using Grpc.Core;
using Microsoft.AspNetCore.Mvc;
using SmartContract.RestBridge.Models;
using SmartContract.RestBridge.Services;

namespace SmartContract.RestBridge.Controllers;

[ApiController]
[Route("api/[controller]")]
public class GraphsController : ControllerBase
{
    private readonly GrpcBlockchainClient _svc;
    private readonly ILogger<GraphsController> _logger;

    public GraphsController(GrpcBlockchainClient svc, ILogger<GraphsController> logger)
    {
        _svc = svc;
        _logger = logger;
    }

    [HttpPost]
    [ProducesResponseType(typeof(Blockchain.CreateGraphResponse), StatusCodes.Status200OK)]
    public async Task<IResult> CreateGraph([FromBody] CreateGraphDto dto, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.CreateGraphAsync(dto.GraphId, (Blockchain.GraphType)dto.GraphType, dto.Description, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpGet]
    [ProducesResponseType(typeof(Blockchain.ListGraphsResponse), StatusCodes.Status200OK)]
    public async Task<IResult> ListGraphs(CancellationToken ct)
    {
        try
        {
            var resp = await _svc.ListGraphsAsync(ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpGet("{graphId}")]
    [ProducesResponseType(typeof(Blockchain.GetGraphInfoResponse), StatusCodes.Status200OK)]
    public async Task<IResult> GetGraphInfo([FromRoute] string graphId, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.GetGraphInfoAsync(graphId, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpPost("{graphId}/verify")]
    [ProducesResponseType(typeof(Blockchain.VerifyGraphResponse), StatusCodes.Status200OK)]
    public async Task<IResult> VerifyGraph([FromRoute] string graphId, CancellationToken ct)
    {
        try
        {
            var resp = await _svc.VerifyGraphAsync(graphId, ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }

    [HttpPost("verify")]
    [ProducesResponseType(typeof(Blockchain.CrossValidateResponse), StatusCodes.Status200OK)]
    public async Task<IResult> CrossValidate(CancellationToken ct)
    {
        try
        {
            var resp = await _svc.CrossValidateAsync(ct);
            return Results.Ok(resp);
        }
        catch (RpcException ex)
        {
            return GrpcBlockchainClient.MapRpcException(ex);
        }
    }
}
