#/usr/bin/env bash

_all_commands()
{
	local commands="compute delete read list store init"
	compgen -W "${commands}" "${COMP_WORDS[1]}"
}

_all_identifiers()
{
	local identifiers=$(find $HOME/.totpc/ -name "*.gpg" | xargs basename -s .gpg)
	compgen -W "${identifiers}" "${COMP_WORDS[2]}"
}

_totpc_completions()
{
	if [[ $COMP_CWORD == 1 ]]; then
		COMPREPLY=($(_all_commands))
	fi
	if [[ $COMP_CWORD -gt 1 ]]; then
		case "${COMP_WORDS[1]}" in
			compute|c|read|r|delete|d)
				COMPREPLY=($(_all_identifiers))
				;;
		esac
	fi
}

complete -F _totpc_completions totpc

