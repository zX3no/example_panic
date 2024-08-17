<!--
Thank you for finding an Internal Compiler Error! 🧊  If possible, try to provide
a minimal verifiable example. You can read "Rust Bug Minimization Patterns" for
how to create smaller examples.
http://blog.pnkfx.org/blog/2019/11/18/rust-bug-minimization-patterns/
-->

### Code

```
📦src
 ┣ 📜lib.rs
 ┗ 📜main.rs
 ```

`main.rs`
```Rust
#![feature(decl_macro)]

pub trait Test {}

struct A {}

// Using this would cause no issues.
// struct B {}

// If this is commented out, the code will run normally.
impl Test for A {}

fn main() {
    pub macro test($t:expr) {
        &$t as &dyn Test
    }

    test!(example_panic::B {});
}
```

`lib.rs`
```rs
pub struct B {}
```

### Meta
<!--
If you're using the stable version of the compiler, you should also check if the
bug also exists in the beta or nightly versions.
-->

`rustc --version --verbose`:
```
rustc 1.82.0-nightly (506052d49 2024-08-16)
binary: rustc
commit-hash: 506052d49d3903ea554e4ce760cc53610cff4ef5
commit-date: 2024-08-16
host: x86_64-pc-windows-msvc
release: 1.82.0-nightly
LLVM version: 19.1.0
```

### Error output

```
error: internal compiler error: compiler\rustc_middle\src\query\plumbing.rs:653:5: `tcx.extern_crate(DefId(0:3 ~ example_panic[331c]::Test))` is not supported for this key;
                                hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.   
                                If that's not the case, extern_crate was likely never assigned to a provider function.

thread 'rustc' panicked at compiler\rustc_middle\src\query\plumbing.rs:653:5:
Box<dyn Any>
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: please make sure that you have updated to the latest nightly

note: please attach the file at `D:\Desktop\rustc_ice\rustc-ice-2024-08-17T07_43_19-28004.txt` to your bug report

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [extern_crate] getting crate's ExternCrateData
#1 [typeck] type-checking `main`
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `example_panic` (bin "example_panic")
```

<!--
Include a backtrace in the code block by setting `RUST_BACKTRACE=1` in your
environment. E.g. `RUST_BACKTRACE=1 cargo build`.
-->
<details><summary><strong>Backtrace</strong></summary>
<p>

```
stack backtrace:
   0:     0x7ffe043608a4 - std::backtrace_rs::backtrace::dbghelp64::trace
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:91
   1:     0x7ffe043608a4 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ffe043608a4 - std::sys::backtrace::_print_fmt
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\sys\backtrace.rs:66
   3:     0x7ffe043608a4 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\sys\backtrace.rs:39
   4:     0x7ffe04391b69 - core::fmt::rt::Argument::fmt
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/core\src\fmt\rt.rs:173
   5:     0x7ffe04391b69 - core::fmt::write
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/core\src\fmt\mod.rs:1178
   6:     0x7ffe04356ad7 - std::io::Write::write_fmt<std::sys::pal::windows::stdio::Stderr>
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\io\mod.rs:1823
   7:     0x7ffe043639b9 - std::panicking::default_hook::closure$1
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\panicking.rs:266
   8:     0x7ffe0436353c - std::panicking::default_hook
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\panicking.rs:293
   9:     0x7ffe05941530 - memchr
  10:     0x7ffe043643cb - alloc::boxed::impl$50::call
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/alloc\src\boxed.rs:2164
  11:     0x7ffe043643cb - std::panicking::rust_panic_with_hook
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\panicking.rs:805
  12:     0x7ffe06ffd913 - <rustc_hir_pretty[3a260ca30a50aa9f]::State>::print_variant
  13:     0x7ffe06fef7f9 - <rustc_hir_pretty[3a260ca30a50aa9f]::State>::print_variant
  14:     0x7ffe06fef7a9 - <rustc_hir_pretty[3a260ca30a50aa9f]::State>::print_variant
  15:     0x7ffe070091f5 - <rustc_errors[ec1c01e799e92f49]::diagnostic::BugAbort as rustc_errors[ec1c01e799e92f49]::diagnostic::EmissionGuarantee>::emit_producing_guarantee     
  16:     0x7ffe06f16c62 - rustc_middle[651375309ecfedd9]::util::bug::bug_fmt
  17:     0x7ffe06ef719d - rustc_middle[651375309ecfedd9]::ty::consts::const_param_default
  18:     0x7ffe06ef6fb6 - rustc_middle[651375309ecfedd9]::ty::consts::const_param_default
  19:     0x7ffe06f16b62 - rustc_middle[651375309ecfedd9]::util::bug::bug_fmt
  20:     0x7ffe06f1c3fb - <rustc_middle[651375309ecfedd9]::query::on_disk_cache::CacheEncoder as rustc_span[a2229aa67142df86]::SpanEncoder>::encode_def_index
  21:     0x7ffe06eb69c4 - <rustc_middle[651375309ecfedd9]::mir::interpret::allocation::ConstAllocation as core[aad56fe385e00b24]::fmt::Debug>::fmt
  22:     0x7ffe03f31233 - <dyn std[a7dfac72b2651342]::io::Write as nu_ansi_term[4e6e1c82295ec14d]::write::AnyWrite>::write_str
  23:     0x7ffe04e17ea7 - rustc_ty_utils[eb7b684e3a64a7da]::ty::self_ty_of_trait_impl_enabling_order_dep_trait_object_hack
  24:     0x7ffe03f50c55 - rustc_query_impl[47fb5b629686e581]::query_system
  25:     0x7ffe05194ad2 - rustc_const_eval[62c94aaddb31f877]::const_eval::eval_queries::eval_to_allocation_raw_provider
  26:     0x7ffe0687e1d3 - <rustc_const_eval[62c94aaddb31f877]::errors::WriteThroughImmutablePointer as rustc_errors[ec1c01e799e92f49]::diagnostic::LintDiagnostic<()>>::decorate_lint
  27:     0x7ffe068faca6 - <rustc_trait_selection[3b8eb77a134af652]::error_reporting::TypeErrCtxt>::fuzzy_match_tys
  28:     0x7ffe06905866 - <rustc_trait_selection[3b8eb77a134af652]::error_reporting::TypeErrCtxt>::is_recursive_obligation
  29:     0x7ffe068f5152 - <rustc_trait_selection[3b8eb77a134af652]::error_reporting::TypeErrCtxt>::report_selection_error
  30:     0x7ffe0698de21 - <rustc_trait_selection[3b8eb77a134af652]::error_reporting::traits::on_unimplemented::WrappedParserError as rustc_errors[ec1c01e799e92f49]::diagnostic::LintDiagnostic<()>>::decorate_lint
  31:     0x7ffe06949de4 - <rustc_trait_selection[3b8eb77a134af652]::error_reporting::TypeErrCtxt>::report_fulfillment_errors
  32:     0x7ffe045b8ef1 - rustc_hir_typeck[10cbc01b29f35403]::typeck
  33:     0x7ffe04f2e16b - rustc_query_impl[47fb5b629686e581]::plumbing::query_key_hash_verify_all
  34:     0x7ffe04e6b5e8 - rustc_ty_utils[eb7b684e3a64a7da]::ty::self_ty_of_trait_impl_enabling_order_dep_trait_object_hack
  35:     0x7ffe04f367a4 - rustc_query_impl[47fb5b629686e581]::plumbing::query_key_hash_verify_all
  36:     0x7ffe0462ccb2 - <rustc_hir_typeck[10cbc01b29f35403]::upvar::InferBorrowKind as rustc_hir_typeck[10cbc01b29f35403]::expr_use_visitor::Delegate>::borrow
  37:     0x7ffe046a8f10 - rustc_hir_analysis[69b0ed4a2515fdd9]::check_crate
  38:     0x7ffe043d3c11 - rustc_interface[4bcdf5dfdcc28355]::passes::resolver_for_lowering_raw
  39:     0x7ffe013e84f7 - rustc_interface[4bcdf5dfdcc28355]::passes::analysis
  40:     0x7ffe03f35b7b - <dyn std[a7dfac72b2651342]::io::Write as nu_ansi_term[4e6e1c82295ec14d]::write::AnyWrite>::write_str
  41:     0x7ffe03e4c35a - rustc_ty_utils[eb7b684e3a64a7da]::ty::adt_sized_constraint
  42:     0x7ffe03f3b303 - rustc_query_impl[47fb5b629686e581]::query_system
  43:     0x7ffe013a3258 - _rust_alloc_error_handler
  44:     0x7ffe0139edcf - _rust_alloc_error_handler
  45:     0x7ffe013a8ccb - _rust_alloc_error_handler
  46:     0x7ffe0437596d - alloc::boxed::impl$48::call_once
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/alloc\src\boxed.rs:2150
  47:     0x7ffe0437596d - alloc::boxed::impl$48::call_once
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/alloc\src\boxed.rs:2150
  48:     0x7ffe0437596d - std::sys::pal::windows::thread::impl$0::new::thread_start
                               at /rustc/506052d49d3903ea554e4ce760cc53610cff4ef5\library/std\src\sys\pal\windows\thread.rs:55
  49:     0x7ffe99517374 - BaseThreadInitThunk
  50:     0x7ffe9b2dcc91 - RtlUserThreadStart
```

</p>
</details>
