import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/inputs/dropdowns/dropdown-select";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    label: string,
    error: string,
    value: string,
    placeholder: string,
    open: boolean,
    maxChildrenHeight: number,
    //Just used for demo
    count: number,
    width: number,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    value: "world",
    placeholder: "select something",
    error: "",
    open: true,
    count: 3,
    maxChildrenHeight: 400,
    width: 300,
}

export const DropdownSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, width, ...dropdownProps} = props

    return `
        <div style="width: ${width}px">
            <dropdown-select ${argsToAttrs(dropdownProps)}>
                ${mapToString(arrayCount(count), i => {
                    return `<li-check>item ${i}</li-check>`;
                })}
            </dropdown-select>
        </div>
    `;
}

DropdownSelect.args = DEFAULT_ARGS;
