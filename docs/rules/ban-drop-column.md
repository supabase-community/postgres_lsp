# banDropColumn
**Diagnostic Category: `lint/safety/banDropColumn`**

**Since**: `vnext`
> [!NOTE]
> - This rule is recommended. A diagnostic error will appear when linting your code.

Sources: 
- Inspired from: <a href="https://squawkhq.com/docs/ban-drop-column" target="_blank"><code>squawk/ban-drop-column</code></a>

## Description
Dropping a column may break existing clients.

Update your application code to no longer read or write the column.

You can leave the column as nullable or delete the column once queries no longer select or modify the column.

## Examples

### Invalid

```sql
alter table test drop column id;
```

<pre class="language-text"><code class="language-text">code-block.sql <a href="https://pglt.dev/linter/rules/ban-drop-column">lint/safety/banDropColumn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━<br /><br />  <strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Dropping a column may break existing clients.</span><br />  <br />  <strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">You can leave the column as nullable or delete the column once queries no longer select or modify the column.</span><br />  <br /></code></pre>

## How to configure
```toml title="pglt.toml"
[linter.rules.safety]
banDropColumn = "error"

```
