use super::builder::{constrained_rule, date, fact, pred, s, string, Atom, Fact, Rule};
use super::Biscuit;
use crate::datalog::{Constraint, ConstraintKind, IntConstraint};
use crate::error;
use std::time::SystemTime;

pub struct Verifier<'a> {
    token: &'a Biscuit,
    facts: Vec<Fact>,
    rules: Vec<Rule>,
    caveats: Vec<Rule>,
}

impl<'a> Verifier<'a> {
    pub(crate) fn new(token: &'a Biscuit) -> Self {
        Verifier {
            token,
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_caveat(&mut self, caveat: Rule) {
        self.caveats.push(caveat);
    }

    pub fn add_resource(&mut self, resource: &str) {
        self.facts
            .push(fact("resource", &[s("ambient"), string(resource)]));
    }

    pub fn add_operation(&mut self, operation: &str) {
        self.facts
            .push(fact("operation", &[s("ambient"), s(operation)]));
    }

    pub fn set_time(&mut self) {
        self.facts.retain(|f| f.0.name != "time");

        self.facts
            .push(fact("time", &[s("ambient"), date(&SystemTime::now())]));
    }

    pub fn revocation_check(&mut self, ids: &[i64]) {
        let caveat = constrained_rule(
            "revocation_check",
            &[Atom::Variable(0)],
            &[pred("revocation_id", &[Atom::Variable(0)])],
            &[Constraint {
                id: 0,
                kind: ConstraintKind::Int(IntConstraint::NotIn(ids.iter().cloned().collect())),
            }],
        );
        self.add_caveat(caveat);
    }

    pub fn verify(&self) -> Result<(), error::Logic> {
        let mut symbols = self.token.symbols.clone();

        let mut ambient_facts = vec![];
        let mut ambient_rules = vec![];
        let mut ambient_caveats = vec![];

        for fact in self.facts.iter() {
            ambient_facts.push(fact.convert(&mut symbols));
        }

        for rule in self.rules.iter() {
            ambient_rules.push(rule.convert(&mut symbols));
        }

        for caveat in self.caveats.iter() {
            ambient_caveats.push(caveat.convert(&mut symbols));
        }

        self.token.check(&symbols, ambient_facts, ambient_rules, ambient_caveats)
    }
}
