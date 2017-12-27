# `before_deploy` phase: here we package the build artifacts

set -ex

. $(dirname $0)/utils.sh

# Generate artifacts for release
mk_artifacts() {
    cargo build --target $TARGET --release
}

mk_tarball() {
    # create a "staging" directory
    local td=$(mktempd)
    local out_dir=$(pwd)
    local name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
    local gcc_prefix="$(gcc_prefix)"
    mkdir "${td:?}/${name}"
    mkdir "$td/$name/complete"

    cp target/$TARGET/release/djs "$td/$name/djs"
    ${gcc_prefix}strip "$td/$name/djs"
    #cp {doc/djs.1,README.md,UNLICENSE,COPYING,LICENSE-MIT} "$td/$name/"
    cp {README.md} "$td/$name/"
    cp \
      target/$TARGET/release/build/djs-*/out/{djs.bash-completion,djs.fish,_djs.ps1} \
      "$td/$name/complete/"
    #cp complete/_djs "$td/$name/complete/"

    pushd $tg
    tar czf "$out_dir/$name.tar.gz" *
    popd
    rm -r $td
}

main() {
    mk_artifacts
    mk_tarball
}

main
