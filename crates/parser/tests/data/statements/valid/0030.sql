CREATE POLICY account_managers ON accounts TO managers
    USING (manager = current_user);
