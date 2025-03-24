import rnix_python

nix_code = '''
let
  example = False;
  another = False;
in
  example
'''

# Suppose the key part is 'example = ' and you want to update the query part after it.
# This call will leave 'example = ' intact and replace everything that follows (in that node)
# with '"new_query_part"'.
updated_code = rnix_python.replace_value(nix_code, 'example', 'True')
print(updated_code)

