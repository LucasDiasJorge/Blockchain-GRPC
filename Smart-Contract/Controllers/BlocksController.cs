using Grpc.Core;
using Microsoft.AspNetCore.Mvc;
using SmartContract.RestBridge.Models;
using SmartContract.RestBridge.Services;

namespace SmartContract.RestBridge.Controllers;

[ApiController]
[Route("api/graphs/{graphId}/[controller]")]
public class BlocksController : ControllerBase
{
    private readonly GrpcBlockchainClient _svc;

    public BlocksController(GrpcBlockchainClient svc)
    {
        _svc = svc;
    }

    [HttpPost]
    [ProducesResponseType(typeof(Blockchain.AddBlockResponse), StatusCodes.Status200OK)]
    public async Task<IResult> AddBlock([FromRoute] string graphId, [FromBody] AddBlockDto dto, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.AddBlockAsync(graphId, dto.Data, dto.CrossReferences, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpGet]
    [ProducesResponseType(typeof(Blockchain.GetBlockRangeResponse), StatusCodes.Status200OK)]
    public async Task<IResult> GetBlockRange([FromRoute] string graphId, [FromQuery] RangeParams query, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.GetBlockRangeAsync(graphId, query.StartHeight, query.EndHeight, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpGet("latest")]
    [ProducesResponseType(typeof(Blockchain.GetBlockResponse), StatusCodes.Status200OK)]
    public async Task<IResult> GetLatest([FromRoute] string graphId, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.GetLatestBlockAsync(graphId, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpGet("{hash}")]
    [ProducesResponseType(typeof(Blockchain.GetBlockResponse), StatusCodes.Status200OK)]
    public async Task<IResult> GetByHash([FromRoute] string graphId, [FromRoute] string hash, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.GetBlockAsync(graphId, hash, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }
}
