-- expect_only_lint/safety/addingRequiredField
alter table test
add column c int not null;