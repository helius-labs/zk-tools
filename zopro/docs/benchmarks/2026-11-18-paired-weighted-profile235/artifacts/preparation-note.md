The first manual profile-helper compile omitted the existing rustc_demangle
--extern argument and failed with E0433 before producing a runnable helper.
The invocation was corrected to link the already available dependency. No
candidate/source algorithm, SDK, protocol or successful capture was changed
or repeated. The initial diagnostic remains in the tool transcript.
