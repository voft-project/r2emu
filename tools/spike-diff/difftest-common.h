#ifndef _DIFFTEST_COMMON_H
#define _DIFFTEST_COMMON_H

#include <generated/autoconf.h>
#include <stdbool.h>
#include <stdint.h>

#ifdef CONFIG_ISA32
typedef uint32_t word_t;
typedef int32_t sword_t;

#else
typedef uint64_t word_t;
typedef int64_t sword_t;
#endif

#if CONFIG_MBASE + CONFIG_MSIZE > 0x100000000ul
#define PMEM64 1
#endif
#ifdef PMEM64
typedef uint64_t paddr_t;
#else
typedef uint32_t paddr_t;
#endif

typedef word_t vaddr_t;
typedef uint16_t ioaddr_t;

#endif