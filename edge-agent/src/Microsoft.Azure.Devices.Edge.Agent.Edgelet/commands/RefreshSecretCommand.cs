using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Azure.Devices.Edge.Agent.Core;

namespace Microsoft.Azure.Devices.Edge.Agent.Edgelet.Commands
{
    public class RefreshSecretCommand : ICommand
    {
        readonly IModule module;
        readonly ISecretManager secretManager;
        readonly string secretId;

        public RefreshSecretCommand(ISecretManager secretManager, IModule module, string secretId)
        {
            this.secretManager = secretManager;
            this.module = module;
            this.secretId = secretId;
        }

        public string Id => $"RefreshSecret({this.module.Name}, {this.secretId})";

        public Task ExecuteAsync(CancellationToken token) => this.secretManager.GetSecretAsync(this.module.Name, this.secretId);

        public Task UndoAsync(CancellationToken token) => Task.CompletedTask;

        public string Show() => $"Refresh secret {this.secretId} for module {this.module.Name}";
    }
}
