// Copyright 2020-2021 The FuseQuery Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datavalues::DataValueLogicOperator;
use crate::error::FuseQueryResult;
use crate::functions::logics::LogicFunction;
use crate::functions::IFunction;
use crate::sessions::FuseQueryContextRef;

pub struct LogicAndFunction;

impl LogicAndFunction {
    pub fn try_create_func(
        _ctx: FuseQueryContextRef,
        args: &[Box<dyn IFunction>],
    ) -> FuseQueryResult<Box<dyn IFunction>> {
        LogicFunction::try_create_func(DataValueLogicOperator::And, args)
    }
}
