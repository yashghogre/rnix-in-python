import rnix_python

'''
nix_code = """
{ config, pkgs, ... }: {
  environment.systemPackages = [ pkgs.htop ];
}
"""
'''

with open('config/yash_configuration.nix') as f:
    conf = f.read()

result = rnix_python.parse_nix(conf)
print("Root node kind:", result["root_kind"])
#print("Root node text:", result["root_text"])
#print("Number of errors:", result["error_count"])
