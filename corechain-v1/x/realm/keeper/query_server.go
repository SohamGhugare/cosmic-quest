package keeper

import (
	"context"
	"fmt"

	"cosmossdk.io/orm/types/ormerrors"
	sdk "github.com/cosmos/cosmos-sdk/types"

	apiv1 "github.com/rollchains/corechain/api/realm/v1"
	"github.com/rollchains/corechain/x/realm/types"
)

var _ types.QueryServer = Querier{}

type Querier struct {
	Keeper
}

func NewQuerier(keeper Keeper) Querier {
	return Querier{Keeper: keeper}
}

func (k Querier) Params(c context.Context, req *types.QueryParamsRequest) (*types.QueryParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)

	p, err := k.Keeper.Params.Get(ctx)
	if err != nil {
		return nil, err
	}

	return &types.QueryParamsResponse{Params: &p}, nil
}

// GetRealm implements types.QueryServer.
func (k Querier) GetRealm(goCtx context.Context, req *types.QueryGetRealmRequest) (*types.QueryGetRealmResponse, error) {
	realm, err := k.OrmDB.RealmTable().Get(goCtx, req.Id)
	if err != nil {
		if ormerrors.IsNotFound(err) {
			return nil, fmt.Errorf("realm with ID %d not found", req.Id)
		}
		return nil, err
	}
	convertedRealm := &types.Realm{
		Id:   realm.Id,
		Name: realm.Name,
	}
	return &types.QueryGetRealmResponse{Realm: convertedRealm}, nil
}

// AllRealm implements types.QueryServer.
func (k Querier) AllRealm(goCtx context.Context, req *types.QueryAllRealmRequest) (*types.QueryAllRealmResponse, error) {
	var realms []*apiv1.Realm
	it, err := k.OrmDB.RealmTable().List(goCtx, &apiv1.RealmPrimaryKey{})
	if err != nil {
		return nil, err
	}
	defer it.Close()

	for it.Next() {
		realm, err := it.Value()
		if err != nil {
			return nil, err
		}
		realms = append(realms, realm)
	}

	var convertedRealms []*types.Realm
	for _, realm := range realms {
		convertedRealms = append(convertedRealms, &types.Realm{
			Id:   realm.Id,
			Name: realm.Name,
		})
	}
	return &types.QueryAllRealmResponse{Realms: convertedRealms}, nil
}
