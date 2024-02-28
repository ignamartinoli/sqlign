(source_file
 (select_statement
  (select_clause
   (select_clause_body (identifier) (alias (identifier))))
  (from_clause (identifier))
  (where_clause
   (binary_expression left: (dotted_name (identifier) (identifier))
                      right: (select_subexpression
                              (select_statement
                               (select_clause (select_clause_body (identifier)))
                               (from_clause (identifier))
                               (where_clause (binary_expression left: (identifier)
                                                                right: (string content: (content)))))))))
 (select_statement
  (select_clause (select_clause_body (identifier)))
  (from_clause (identifier))))
