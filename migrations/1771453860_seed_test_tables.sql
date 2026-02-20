INSERT INTO tables (id, name, status, settings) VALUES
  (gen_random_uuid(), 'Table Alpha',   'open',   '{"min_bet": 10, "max_bet": 500, "max_players": 5, "max_observers": 10}'),
  (gen_random_uuid(), 'Table Bravo',   'open',   '{"min_bet": 25, "max_bet": 500, "max_players": 7, "max_observers": 15}'),
  (gen_random_uuid(), 'Table Charlie',   'open',   '{"min_bet": 25, "max_bet": 1000, "max_players": 7, "max_observers": 15}')

