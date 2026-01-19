#ifndef _DIFFTEST_DEF_H
#define _DIFFTEST_DEF_H

#include <generated/autoconf.h>
#include <stdint.h>

#ifdef CONFIG_ISA32
#define RISCV_GPR_TYPE uint32_t
#elif CONFIG_ISA64
#define RISCV_GPR_TYPE uint64_t
#else
#error Unsupported ISA
#endif

#define RISCV_GRP_NUM 32
#define DIFFTEST_REG_SIZE (sizeof(RISCV_GPR_TYPE) * (RISCV_GPR_NUM + 1)) // GPRs + pc
#define __EXPORT __attribute__((visibility("default")))

enum { DIFFTEST_TO_DUT, DIFFTEST_TO_REF };

#endif