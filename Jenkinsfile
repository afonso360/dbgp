
node('master') {
        try {
            stage 'Checkout'
                checkout scm
            stage 'Version Check'
                sh 'rustc --version'
                sh 'cargo --version'
            stage 'Build dbgp'
                sh '(cd dbgp && nix-shell --run "cargo build --verbose")'
            stage 'Test dbgp'
                sh '(cd dbgp && nix-shell --run "cargo test --verbose")'
            stage 'Doc dbgp'
                sh '(cd dbgp && nix-shell --run "cargo doc --verbose")'
            stage 'Build dbgp-capi'
                sh '(cd dbgp_capi && nix-shell --run "cargo build --verbose")'
            stage 'Test dbgp-capi'
                sh '(cd dbgp_capi && nix-shell --run "cargo test --verbose")'
            stage 'Doc dbgp-capi'
                sh '(cd dbgp_capi && nix-shell --run "cargo doc --verbose")'
            currentBuild.result = "SUCCESS"
        } catch (err) {
            currentBuild.result = "FAILURE"
            throw err
        }
}
