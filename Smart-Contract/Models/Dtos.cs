using System.ComponentModel.DataAnnotations;

namespace SmartContract.RestBridge.Models;

public record CreateGraphDto(
    [property: Required, MinLength(1)] string GraphId,
    [property: Required] int GraphType,
    [property: Required] string Description
);

public record AddBlockDto(
    [property: Required] string Data,
    [property: Required] List<string> CrossReferences
);

public record RangeParams(
    [property: Required] ulong StartHeight,
    [property: Required] ulong EndHeight
);
