package module

import (
	autocliv1 "cosmossdk.io/api/cosmos/autocli/v1"
	modulev1 "github.com/rollchains/corechain/api/realm/v1"
)

// AutoCLIOptions implements the autocli.HasAutoCLIConfig interface.
func (am AppModule) AutoCLIOptions() *autocliv1.ModuleOptions {
	return &autocliv1.ModuleOptions{
		Query: &autocliv1.ServiceCommandDescriptor{
			Service: modulev1.Query_ServiceDesc.ServiceName,
			RpcCommandOptions: []*autocliv1.RpcCommandOptions{
				{
					RpcMethod: "GetRealm",
					Use:       "get [id]",
					Short:     "Get Realm by ID",
					PositionalArgs: []*autocliv1.PositionalArgDescriptor{
						{ProtoField: "id"},
					},
				},
				{
					RpcMethod: "AllRealm",
					Use:       "all",
					Short:     "Get All Realm",
				},
				{
					RpcMethod: "Params",
					Use:       "params",
					Short:     "Query the current consensus parameters",
				},
			},
		},
		Tx: &autocliv1.ServiceCommandDescriptor{
			Service: modulev1.Msg_ServiceDesc.ServiceName,
			RpcCommandOptions: []*autocliv1.RpcCommandOptions{
				{
					RpcMethod: "CreateRealm",
					Use:       "create [name]",
					Short:     "Create new realm",
					PositionalArgs: []*autocliv1.PositionalArgDescriptor{
						{ProtoField: "name"},
					},
				},
				{
					RpcMethod: "UpdateParams",
					Skip:      false, // set to true if authority gated
				},
			},
		},
	}
}
