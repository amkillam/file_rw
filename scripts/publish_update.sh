#!/usr/bin/env sh

ensure_git_clean() {
	if [ -n "$(git status --porcelain)" ]; then
		echo "Git status is not clean. Please commit all changes before publishing."
		exit 1
	fi
}

get_current_version() {
	version_line = $(grep -io "version = \"[0-9]\+\.[0-9]\+\.[0-9]\+\"" Cargo.toml)
	echo "${version_line}" | cut -d '"' -f 2
}

gen_new_version() {
	old_version=$1
	release_type="$2"

	major=$(echo ${old_version} | cut -d '.' -f 1)
	minor=$(echo ${old_version} | cut -d '.' -f 2)
	patch=$(echo ${old_version} | cut -d '.' -f 3)

	if [ "${release_type}" = "major" ]; then
		major=$((major + 1))
		minor=0
		patch=0
	elif [ "${release_type}" = "minor" ]; then
		minor=$((minor + 1))
		patch=0
	elif [ "${release_type}" = "patch" ]; then
		patch=$((patch + 1))
	else
		echo "Unknown release type: ${release_type}"
		exit 1
	fi

	echo "${major}.${minor}.${patch}"
}

update_readme_version() {
	old_version=$1
	new_version=$2
	sed -i "s/${old_version}/${new_version}/g" README.md
}

update_cargo_version() {
	old_version=$1
	new_version=$2
	sed -i "s/version = \"${old_version}\"/version = \"${new_version}\"/g" Cargo.toml
}

print_help() {
	echo "Usage: $0 <release_type>"
	echo "  release_type: major, minor, patch"
}

main() {
	release_type="$1"

	if [ -z "${release_type}" ]; then
		print_help
		exit 1
	fi

	current_version=$(get_current_version)
	new_version="$(gen_new_version ${current_version} ${release_type})"

	echo "Current version: ${current_version}"
	echo "New version: ${new_version}"

	update_readme_version "${current_version}" "${new_version}"
	update_cargo_version "${current_version}" "${new_version}"

	ensure_git_clean
	git add README.md Cargo.toml
	git commit -m "Release ${new_version}"
	git tag -a "v${new_version}" -m "Release ${new_version}"
	git push origin master --tags

	cargo publish

}

main "$@"
