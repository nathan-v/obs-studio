#include <stdbool.h>

/* Original cf preprocessor compiled unchanged with independent C dependencies. */
#define cf_preprocessor_init oracle_cf_preprocessor_init
#define cf_preprocessor_free oracle_cf_preprocessor_free
#define cf_preprocess oracle_cf_preprocess
#define cf_preprocessor_add_def oracle_cf_preprocessor_add_def
#define cf_preprocessor_remove_def oracle_cf_preprocessor_remove_def
#define cf_preprocess_get_def_value oracle_cf_preprocess_get_def_value
#define cf_lexer_init oracle_cf_lexer_init
#define cf_lexer_free oracle_cf_lexer_free
#define cf_lexer_lex oracle_cf_lexer_lex
#define strref_cmp oracle_strref_cmp
#define strref_cmp_strref oracle_strref_cmp_strref
#define lexer_getstroffset oracle_lexer_getstroffset
#define error_data_add oracle_error_data_add
#define dstr_safe_printf oracle_dstr_safe_printf
#define dstr_init_copy_strref oracle_dstr_init_copy_strref
#define dstr_copy_strref oracle_dstr_copy_strref
#define dstr_ncopy oracle_dstr_ncopy
#define dstr_insert_dstr oracle_dstr_insert_dstr
#define dstr_mid oracle_dstr_mid
#define os_fopen oracle_os_fopen
#define os_fread_utf8 oracle_os_fread_utf8
/* Keep the original pointer-difference expression unchanged. */
#if defined(__clang__) || defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wsign-conversion"
#endif
struct cf_preprocessor;
struct strref;
bool oracle_cf_preprocess_get_def_value(struct cf_preprocessor *, const struct strref *, int *);

#include "util/cf-lexer.c.h"
#if defined(__clang__) || defined(__GNUC__)
#pragma GCC diagnostic pop
#endif

size_t oracle_cf_def_layout(size_t index);
size_t oracle_cf_def_layout(size_t index)
{
	const size_t fields[] = {sizeof(struct cf_def),           _Alignof(struct cf_def),
				 offsetof(struct cf_def, name),   offsetof(struct cf_def, params),
				 offsetof(struct cf_def, tokens), offsetof(struct cf_def, macro)};
	return fields[index];
}
