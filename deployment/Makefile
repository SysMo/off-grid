SGCLI_PATH=/data/Workspace/SysMo/sysmograph/sg-cli
SGCLI = ${SGCLI_PATH}/target/release/sg_cli
NAMESPACE = offgrid

cli/build:
	clear
	cd ${SGCLI_PATH} && cargo build --release

namespace:
	kubectl apply -f namespace.yaml

# mosquitto/cert/server: cli/build
# 	${SGCLI} kube gen-cert --in mosquitto/server_cert.yaml --out mosquitto/derived/
# mosquitto/cert/deploy: mosquitto/cert/server
# 	kubectl apply -f mosquitto/derived/mosquitto-server-cert-secret.yaml

kubectl/namespace:
	kubectl apply -f namespace.yaml

mosquitto/gen/config: cli/build
	${SGCLI} kube gen-config --in mosquitto/config-objects.yaml --out mosquitto/mosquitto-chart/templates

mosquitto/gen/deploy: cli/build
	${SGCLI} mosquitto helm-generate --in mosquitto/install.yaml --out mosquitto/mosquitto-chart/templates

mosquitto/deploy: mosquitto/gen/config mosquitto/gen/deploy
	helm upgrade -i --namespace ${NAMESPACE} mosquitto ./mosquitto/mosquitto-chart

mosquitto/remove: 
	helm uninstall --namespace ${NAMESPACE} mosquitto



grafana/deploy/gen: cli/build
	${SGCLI} grafana helm-generate --in grafana/install.yaml --out grafana/derived/helm_vars.yaml

grafana/deploy: grafana/deploy/gen
	helm upgrade -i --namespace ${NAMESPACE} -f grafana/derived/helm_vars.yaml grafana grafana/grafana

grafana/deploy/remove:
	helm uninstall --namespace ${NAMESPACE} grafana

