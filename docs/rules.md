# Rules

Below the list of rules supported by Postgres Language Tools, divided by group. Here's a legend of the emojis:

- The icon <span class='inline-icon' title="This rule is recommended"><Icon name="approve-check-circle"x label="This rule is recommended" /></span> indicates that the rule is part of the recommended rules.

[//]: # (BEGIN RULES_INDEX)

## Safety

Rules that detect potential safety issues in your code.
| Rule name | Description | Properties |
| --- | --- | --- |
| [addingRequiredField](./rules/adding-required-field) | Adding a new column that is NOT NULL and has no default value to an existing table effectively makes it required. |  |
| [banDropColumn](./rules/ban-drop-column) | Dropping a column may break existing clients. | <span class='inline-icon' title="This rule is recommended" ><Icon name="approve-check-circle" size="1.2rem" label="This rule is recommended" /></span> |
| [banDropNotNull](./rules/ban-drop-not-null) | Dropping a NOT NULL constraint may break existing clients. | <span class='inline-icon' title="This rule is recommended" ><Icon name="approve-check-circle" size="1.2rem" label="This rule is recommended" /></span> |
| [banDropTable](./rules/ban-drop-table) | Dropping a table may break existing clients. | <span class='inline-icon' title="This rule is recommended" ><Icon name="approve-check-circle" size="1.2rem" label="This rule is recommended" /></span> |

[//]: # (END RULES_INDEX)


