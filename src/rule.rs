/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: rule.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-14 18:38:14
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 19:48:45
 * @Description: Rules for the parameters of a function
 */

use crate::rotype::RoType;

// what the rule is looking for
#[derive(Clone)]
pub enum RuleType {
	EQUAL(RoType),
	GREATER(usize),
	LESS(usize),
	AND(RuleDuo),
	OR(RuleDuo),
	XOR(RuleDuo),
	NOT(Box<FunctionRule>),
	CONTAIN(Vec<RoType>)
}

#[derive(Clone)]
pub struct FunctionRule {
	pub parameter: String,
	pub ruletype: RuleType
}

// two rules joined for a RuleType
#[derive(Clone)]
pub struct RuleDuo(Box<FunctionRule>, Box<FunctionRule>);