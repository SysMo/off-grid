SGCLI_PATH=/data/Workspace/SysMo/sysmograph/sg-cli
SGCLI = ${SGCLI_PATH}/target/release/sg_cli
NAMESPACE = offgrid

cli/build:
	clear
	cd ${SGCLI_PATH} && cargo build --release


kubectl/namespace:
	kubectl apply -f namespace.yaml

mosquitto/deploy/gen: cli/build
	${SGCLI} mosquitto helm-generate --in mosquitto/install.yaml --out mosquitto/derived/helm_vars.yaml


grafana/deploy/config: cli/build
	${SGCLI} grafana helm-generate --in grafana/install.yaml --out grafana/derived/helm_vars.yaml

grafana/deploy/upgrade: grafana/deploy/config
	helm upgrade --namespace ${NAMESPACE} -f grafana/derived/helm_vars.yaml grafana grafana/grafana

grafana/deploy/remove:
	helm uninstall --namespace ${NAMESPACE} grafana

grafana/deploy: grafana/deploy/config
	helm install --namespace ${NAMESPACE} -f grafana/derived/helm_vars.yaml grafana grafana/grafana