export { allocate, deallocate } from "./memory";

export function instantiate(env_ptr: usize, info_ptr: usize, msg_ptr: usize): usize {
    return 0
}

export function execute(env_ptr: usize, info_ptr: usize, msg_ptr: usize): usize {
    return 0;
}

export function query(env_ptr: usize, msg_ptr: usize): usize {
    return 0;
}

export function interface_version_8(): void { }