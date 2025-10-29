using Grpc.Core;

namespace SmartContract.RestBridge.Services;

public class GrpcBlockchainClient
{
    private readonly Blockchain.BlockchainService.BlockchainServiceClient _client;
    private readonly ILogger<GrpcBlockchainClient> _logger;

    public GrpcBlockchainClient(Blockchain.BlockchainService.BlockchainServiceClient client, ILogger<GrpcBlockchainClient> logger)
    {
        _client = client;
        _logger = logger;
    }

    public async Task<Blockchain.CreateGraphResponse> CreateGraphAsync(string graphId, Blockchain.GraphType graphType, string description, CancellationToken ct)
        => await _client.CreateGraphAsync(new Blockchain.CreateGraphRequest
        {
            GraphId = graphId,
            GraphType = graphType,
            Description = description
        }, cancellationToken: ct);

    public async Task<Blockchain.ListGraphsResponse> ListGraphsAsync(CancellationToken ct)
        => await _client.ListGraphsAsync(new Blockchain.ListGraphsRequest(), cancellationToken: ct);

    public async Task<Blockchain.GetGraphInfoResponse> GetGraphInfoAsync(string graphId, CancellationToken ct)
        => await _client.GetGraphInfoAsync(new Blockchain.GetGraphInfoRequest { GraphId = graphId }, cancellationToken: ct);

    public async Task<Blockchain.VerifyGraphResponse> VerifyGraphAsync(string graphId, CancellationToken ct)
        => await _client.VerifyGraphAsync(new Blockchain.VerifyGraphRequest { GraphId = graphId }, cancellationToken: ct);

    public async Task<Blockchain.CrossValidateResponse> CrossValidateAsync(CancellationToken ct)
        => await _client.CrossValidateGraphsAsync(new Blockchain.CrossValidateRequest(), cancellationToken: ct);

    public async Task<Blockchain.AddBlockResponse> AddBlockAsync(string graphId, string data, IEnumerable<string> refs, CancellationToken ct)
        => await _client.AddBlockAsync(new Blockchain.AddBlockRequest
        {
            GraphId = graphId,
            Data = data,
            CrossReferences = { refs }
        }, cancellationToken: ct);

    public async Task<Blockchain.GetBlockResponse> GetLatestBlockAsync(string graphId, CancellationToken ct)
        => await _client.GetLatestBlockAsync(new Blockchain.GetLatestBlockRequest { GraphId = graphId }, cancellationToken: ct);

    public async Task<Blockchain.GetBlockResponse> GetBlockAsync(string graphId, string hash, CancellationToken ct)
        => await _client.GetBlockAsync(new Blockchain.GetBlockRequest { GraphId = graphId, Hash = hash }, cancellationToken: ct);

    public async Task<Blockchain.GetBlockRangeResponse> GetBlockRangeAsync(string graphId, ulong startHeight, ulong endHeight, CancellationToken ct)
        => await _client.GetBlockRangeAsync(new Blockchain.GetBlockRangeRequest { GraphId = graphId, StartHeight = startHeight, EndHeight = endHeight }, cancellationToken: ct);

    public static IResult MapRpcException(RpcException ex)
    {
        var status = ex.Status.StatusCode;
        var msg = ex.Status.Detail;
        return status switch
        {
            StatusCode.InvalidArgument => Results.BadRequest(new { error = msg }),
            StatusCode.NotFound => Results.NotFound(new { error = msg }),
            StatusCode.Unavailable => Results.Problem(msg, statusCode: 503),
            _ => Results.Problem(msg, statusCode: 500)
        };
    }
}
