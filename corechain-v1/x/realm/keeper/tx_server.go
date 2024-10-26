package keeper

import (
	"context"

	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"

	"cosmossdk.io/errors"
	apiv1 "github.com/rollchains/corechain/api/realm/v1"
	"github.com/rollchains/corechain/x/realm/types"
)

type msgServer struct {
	k Keeper
}

var _ types.MsgServer = msgServer{}

// NewMsgServerImpl returns an implementation of the module MsgServer interface.
func NewMsgServerImpl(keeper Keeper) types.MsgServer {
	return &msgServer{k: keeper}
}

func (ms msgServer) UpdateParams(ctx context.Context, msg *types.MsgUpdateParams) (*types.MsgUpdateParamsResponse, error) {
	if ms.k.authority != msg.Authority {
		return nil, errors.Wrapf(govtypes.ErrInvalidSigner, "invalid authority; expected %s, got %s", ms.k.authority, msg.Authority)
	}

	return nil, ms.k.Params.Set(ctx, msg.Params)
}

// CreateRealm implements types.MsgServer.
func (ms msgServer) CreateRealm(ctx context.Context, msg *types.MsgCreateRealm) (*types.MsgCreateRealmResponse, error) {

	if msg.Name == "" {
		return nil, errors.Wrap(types.ErrInvalidLengthTx, "realm name cannot be empty")
	}
	// id, err := ms.k.GetNextRealmID(ctx)
	// if err != nil {
	// 	return nil, err
	// }
	realm := &apiv1.Realm{
		// Id:   id,
		Name: msg.Name,
	}

	err := ms.k.OrmDB.RealmTable().Save(ctx, realm)
	if err != nil {
		return nil, err
	}

	return &types.MsgCreateRealmResponse{Id: realm.Id}, nil
}
