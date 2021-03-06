using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Azure.Devices.Edge.Agent.Core;

namespace Microsoft.Azure.Devices.Edge.Agent.Edgelet.Commands
{
    public class SetSecretCommand : ICommand
    {
        readonly IModule module;
        readonly ISecretManager secretManager;
        readonly string secretId;
        readonly string secretValue;

        public SetSecretCommand(ISecretManager secretManager, IModule module, string secretId, string secretValue)
        {
            this.secretManager = secretManager;
            this.module = module;
            this.secretId = secretId;
            this.secretValue = secretValue;
        }

        public string Id => $"SetSecret({this.module.Name}, {this.secretId})";

        public Task ExecuteAsync(CancellationToken token) => this.secretManager.SetSecretAsync(this.module.Name, this.secretId, this.secretValue);

        public Task UndoAsync(CancellationToken token) => Task.CompletedTask;

        public string Show() => $"Set secret {this.secretId} for module {this.module.Name}";
    }
}
