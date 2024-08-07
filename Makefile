PYTHON_VER=3.12
VENV_PATH:= ${PWD}/.venv
LIB_PATH = ${PWD}/pylib
BASE_DIR=$(shell pwd)

venv/install_pkgs:
	/bin/bash -c "source $(VENV_PATH)/bin/activate && pip install -r requirements.txt"
	
venv/create:
	python3 -m venv $(VENV_PATH)

venv/add_lib_path:
	echo ${LIB_PATH} > $(VENV_PATH)/lib/python$(PYTHON_VER)/site-packages/smo.pth

setup: venv/create venv/install_pkgs venv/add_lib_path


# deploy/rpi:
# 	jetp check-ssh --playbook deploy/playbooks/test1.yaml --user sysmo\
# 		--inventory deploy/inventory -vvv --threads 30

ansible/role:
	cd deploy && ansible-playbook playbooks/$(role).yaml -vv -e "@options.yaml" -e "role_action=$(action)" -e "proj_dir=$(BASE_DIR)"

morse/deploy/rpi:
	make ansible/role role=morse-edge action=deploy 
