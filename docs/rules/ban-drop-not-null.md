# banDropNotNull
**Diagnostic Category: `lint/safety/banDropNotNull`**
**Since**: `vnext`
:::note
- This rule is recommended. A diagnostic error will appear when linting your code.
:::
Sources: 
- Inspired from: <a href="https://squawkhq.com/docs/ban-drop-not-null" target="_blank"><code>squawk/ban-drop-not-null</code></a>

## Description
Dropping a NOT NULL constraint may break existing clients.

Application code or code written in procedural languages like PL/SQL or PL/pgSQL may not expect NULL values for the column that was previously guaranteed to be NOT NULL and therefore may fail to process them correctly.

You can consider using a marker value that represents NULL. Alternatively, create a new table allowing NULL values, copy the data from the old table, and create a view that filters NULL values.

## Examples

### Invalid

```sql
alter table users alter column email drop not null;
```

<pre class="language-text"><code class="language-text">code-block.sql <a href="https://pglt.dev/linter/rules/ban-drop-not-null">lint/safety/banDropNotNull</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━<br /><br />  <strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Dropping a NOT NULL constraint may break existing clients.</span><br />  <br />  <strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider using a marker value that represents NULL. Alternatively, create a new table allowing NULL values, copy the data from the old table, and create a view that filters NULL values.</span><br />  <br /></code></pre>

## How to configure
```toml title="pglt.toml"
[linter.rules.safety]
banDropNotNull = "error"

```
