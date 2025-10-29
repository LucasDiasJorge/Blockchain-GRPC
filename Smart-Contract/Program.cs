using Grpc.Net.Client;
using Microsoft.AspNetCore.Mvc;
using Microsoft.OpenApi.Models;
using SmartContract.RestBridge.Services;

var builder = WebApplication.CreateBuilder(args);

// Config: gRPC endpoint
var grpcEndpoint = builder.Configuration.GetValue<string>("Grpc:Endpoint") ?? "http://127.0.0.1:50051";

// Controllers and Swagger
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new OpenApiInfo
    {
        Title = "Blockchain REST → gRPC Bridge",
        Version = "v1",
        Description = "REST facade for the Blockchain gRPC API"
    });
});

// gRPC typed client generated from proto (namespace is 'Blockchain')
builder.Services.AddGrpcClient<Blockchain.BlockchainService.BlockchainServiceClient>(o =>
{
    o.Address = new Uri(grpcEndpoint);
});

// App services
builder.Services.AddScoped<GrpcBlockchainClient>();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.MapControllers();

app.Run();
