@unmanaged
export class Region {
    offset: usize;
    capacity: usize;
    length: usize;
}

export function allocate(size: usize): usize {
    const buffer = heap.alloc(size);
    const region: Region = {
        offset: buffer,
        capacity: size,
        length: 0,
    };
    return changetype<usize>(region);
}

export function deallocate(regionPtr: usize): void {
    const offsetPtr = changetype<Region>(regionPtr).offset;
    heap.free(regionPtr);
    heap.free(offsetPtr);
}