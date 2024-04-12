#!/usr/bin/env python3.8
# coding:utf-8
# Copyright (C) 2023 All rights reserved.
# FILENAME:    ~~/src/bitvm/bigint/__init__.py
# VERSION: 	   0.1.0
# CREATED: 	   2024-04-12 22:41
# AUTHOR: 	   Sitt Guruvanich <aekazitt+github@gmail.com>
# DESCRIPTION:
#
# HISTORY:
# *************************************************************

### Standard packages ###
from typing import Optional, Literal

### Third-party modules ###
from pydantic import BaseModel, PositiveInt


class BigInt(BaseModel):
    head: Optional[PositiveInt] = None
    head_offset: Optional[PositiveInt] = None
    n_bits: PositiveInt
    n_limbs: Optional[PositiveInt] = None

    def model_post_init(self, context) -> None:
        print("post init called")
        self.n_limbs = int((self.n_bits + 30 - 1) / 30)
        self.head = int(self.n_bits - (self.n_limbs - 1) * 30)
        self.head_offset = 0x1 << self.head


U254 = Literal[BigInt(n_bits=254)]

__all__ = ("BigInt", "U254")
