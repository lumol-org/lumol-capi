#ifdef __cplusplus
extern "C" {
#endif

#ifndef LUMOL_CAPI_H
#define LUMOL_CAPI_H

/* Generated with cbindgen:0.1.28 */

/* Automatically generated file, do not edit */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  LML_SUCCESS = 0,
  LML_ERROR = 1,
} lml_status;

struct lml__system__private__;
typedef struct lml__system__private__ lml__system__private__;

typedef struct {
  lml__system__private__ *handle;
  size_t natoms;
  double *masses;
  double *charges;
  double (*positions)[3];
  double (*velocities)[3];
} lml_system_t;

/* Automatically generated file, do not edit */

lml_system_t lml_system();

lml_status lml_system_add_particle(lml_system_t *system,
                                   const char *name,
                                   double position[3],
                                   double velocity[3]);

void lml_system_free(lml_system_t system);

/* Automatically generated file, do not edit */

#endif /* LUMOL_CAPI_H */

#ifdef __cplusplus
}
#endif
