# SynapT backlog — Default project standards

Status: To do

Let users configure a standards file once so every new SynapT project starts with the instructions its AI assistant must follow. For VSS-IT, the default source is `ENGINEERING-STANDARDS.md`.

## Acceptance criteria

- Workspace settings let users select a default standards file and define the project instructions that point to it.
- Every new project, including a conversation promoted to a project, automatically lists the file under Project Sources and inherits the associated instructions. Retrying setup does not duplicate the source or instructions.
- The assistant reads and applies the relevant standards when planning, implementing, reviewing, testing, documenting, and deploying. Before marking work complete, it reports deviations and checks it could not verify.
- Core rules stored directly in Project Instructions still apply if the source is unavailable. The assistant reports the missing source and avoids claiming compliance with standards it could not verify.
- Existing projects can opt in from Project Settings. Changes to the workspace default apply to future projects.
