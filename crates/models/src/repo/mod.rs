use crate::repo::notes_repo::NoteRepo;
use crate::repo::scans_repo::ScanRepo;
use crate::repo::selling_partner_key_marketplaces_repo::SellingPartnerKeyMarketplaceRepo;
use crate::repo::selling_partner_key_repo::SellingPartnerKeyRepo;
use crate::repo::users_repo::UserRepo;

mod notes_repo;
mod scans_repo;
mod selling_partner_key_marketplaces_repo;
mod selling_partner_key_repo;
mod users_repo;

#[derive(Clone)]
pub struct Repo {
    pub scans: ScanRepo,
    pub notes: NoteRepo,
    pub users: UserRepo,
    pub keys: SellingPartnerKeyRepo,
    pub key_marketplaces: SellingPartnerKeyMarketplaceRepo,
    pub pool: sqlx::PgPool,
}

impl Repo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            scans: ScanRepo::new(pool.clone()),
            notes: NoteRepo::new(pool.clone()),
            users: UserRepo::new(pool.clone()),
            keys: SellingPartnerKeyRepo::new(pool.clone()),
            key_marketplaces: SellingPartnerKeyMarketplaceRepo::new(pool.clone()),
            pool,
        }
    }
}
