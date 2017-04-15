

def branches = [:]
def names = nodeNames();

for (int i = 0; i < names.size(); i++) {
	def nodeName = names[i];
	branches["node(" + nodeName + ")"] = {
		node(nodeName) {
			stage('Checkout') {
				checkout scm;
			}
			stage('Version') {
				sh 'rustc --version';
				sh 'cargo --version';
			}
			stage('Build') {
				sh '(cd dbgp && cargo build --verbose)';
				sh '(cd dbgp_capi && cargo build --verbose)';
			}
			stage('Test') {
				sh '(cd dbgp && cargo test --verbose)';
				sh '(cd dbgp_capi && cargo test --verbose)';
			}
			stage('Doc') {
				sh '(cd dbgp && cargo doc --verbose)';
				sh '(cd dbgp_capi && cargo doc --verbose)';
			}
		}
	}
}

// Now we trigger all branches
parallel branches

// This method collects a list of Node names from the current Jenkins instance
@NonCPS
def nodeNames() {
	return jenkins.model.Jenkins.instance.nodes.collect { node -> node.name }
}

