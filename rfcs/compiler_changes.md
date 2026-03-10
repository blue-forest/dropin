# RFC policy - the compiler

We have not previously had an RFC system for compiler changes, so policy here is
likely to change as we get the hang of things. We don't want to slow down most
compiler development, but on the other hand we do want to do more design work
ahead of time on large additions and refactorings.

Compiler RFCs will be managed by the compiler sub-team, and tagged `compiler`.
The compiler sub-team will do an initial triage of new PRs within a week of
submission. The result of triage will either be that the PR is assigned to a
member of the sub-team for shepherding, the PR is closed because the sub-team
believe it should be done without an RFC, or closed because the sub-team feel it
should clearly not be done and further discussion is not necessary. We'll follow
the standard procedure for shepherding, final comment period, etc.

Where there is significant design work for the implementation of a language
feature, the preferred workflow is to submit two RFCs - one for the language
design and one for the implementation design. The implementation RFC may be
submitted later if there is scope for large changes to the language RFC.


## Changes which need an RFC

* New lints (these fall under the lang team)
* Large refactorings or redesigns of the compiler
* Changing the API presented to syntax extensions or other compiler plugins in
  non-trivial ways
* The implementation of new language features where there is significant change
  or addition to the compiler. There is obviously some room for interpretation
  about what consitutes a "significant" change and how much detail the
  implementation RFC needs. 
* Any other change which causes backwards incompatible changes to stable
  behaviour of the compiler, language, or libraries


## Changes which don't need an RFC

* Bug fixes, improved error messages, etc.
* Minor refactoring/tidying up
* Implementing language features which have an accepted RFC, where the
  implementation does not significantly change the compiler or require
  significant new design work
* Adding unstable API for tools (note that all compiler API is currently
  unstable)

