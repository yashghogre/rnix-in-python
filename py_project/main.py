import rnix_python

nix_code = '''
let
  example = False;
  another = old_value;
in
  example
'''
updated_code = rnix_python.replace_value(nix_code, "example = False;", "example = True;")
print(updated_code)

