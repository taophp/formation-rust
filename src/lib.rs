/// Helps to manager concurrent versions of things
///
/// Given two versions of a thing T, version A (original) and version B (challenger), where version B is expected to be an updated version of A, Mogulid checks if the update should occurs.
/// In some cases, update should be completly avoid : A and B are not versions of the same thing, B is older than A, A and B are the same.
/// In one case, there is a conflict that could not be solved at this level, update should be canceled and the application should handle the conflict : A and B are divergent versions of a common ancestor.
/// In case of conflict, Mogulid can not tell how the conflict should be resolved. It is the responsability of the application to provide a new version C that solves the issue.
/// To manage all that stuff, Mogulid propose to associate each versions of T with a `Mogulid`. This structure have three members :
///     * `ulid` which identify the associate thing T,
///     * `state` which identify the version A or B with an ulid,
///     * and `history` which stores successiv states in a vector.
/// It is the responsability of the application to store and provide those Mogulid beside things to handle. The application must provide a Mogulid for each version.

use ulid::Ulid;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize)]
pub enum MogulidError {
    NotVersionsOfTheSameObject,
    ChallengerOlderThanOriginal,
    IdenticalVersions,
    Conflict,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Mogulid {
    ulid: Ulid,
    state: Ulid,
    history: Vec<Ulid>
}

#[allow(dead_code)]
impl Mogulid {
    pub fn new(ulid: Ulid, state: Ulid, history: Vec<Ulid>) -> Self {
        Mogulid {
            ulid,
            state,
            history
        }
    }

    pub fn new_start() -> Self {
        Mogulid {
            ulid: Ulid::new(),
            state: Ulid::new(),
            history: vec![]
        }
    }

    pub fn define_new_state(&mut self) {
    }

    pub fn get_ulid(&self) -> Ulid {
        self.ulid
    }

    pub fn get_state(&self) -> Ulid {
        self.state
    }

    pub fn get_history(&self) -> Vec<Ulid> {
        self.history.clone()
    }
}

/// ## Allowing merge of versions
///
/// You can use this function to know if two versions can be merge and, if not, why.
///
/// ```
/// use mogulid::{Mogulid, MogulidError, mogulid_allow_update};
/// use ulid::Ulid;
///
/// let original = Mogulid::new(Ulid::new(), Ulid::new(), vec![Ulid::new(), Ulid::new()]);
/// let challenger_not_same = Mogulid::new(Ulid::new(), Ulid::new(), vec![Ulid::new(), Ulid::new()]);
///
/// // Test: versions of different things
/// assert_eq!(mogulid_allow_update(&original, &challenger_not_same), Err(MogulidError::NotVersionsOfTheSameObject));
///
/// // Test: Challenger older
/// let challenger_older = Mogulid::new(original.get_ulid(), original.get_history()[0], vec![]);
/// assert_eq!(mogulid_allow_update(&original, &challenger_older), Err(MogulidError::ChallengerOlderThanOriginal));
///
/// // Test: Conflict
/// let challenger_diverge_older = Mogulid::new(original.get_ulid(), Ulid::new(), vec![original.get_history()[0]]);
/// assert_eq!(mogulid_allow_update(&original, &challenger_diverge_older), Err(MogulidError::Conflict));
///
/// // Test: identical versions
/// assert_eq!(mogulid_allow_update(&original, &original), Err(MogulidError::IdenticalVersions));
///
/// // Test: allowing update
/// let mut challenger_allowed = Mogulid::new(original.get_ulid(), Ulid::new(), original.get_history());
/// challenger_allowed.define_new_state();
/// assert_eq!(mogulid_allow_update(&original, &challenger_allowed), Ok(()));
/// ```
#[allow(dead_code)]
pub fn mogulid_allow_update(_original:&Mogulid, _challenger:&Mogulid) -> Result<(), MogulidError> {
  Ok(())
}

/// ## Merging versions
///
/// When two versions are valid and must be merged, we want to merge their histories to get a new Mogulid
///
/// ```
/// use mogulid::{Mogulid, mogulid_merge};
/// use ulid::Ulid;
///
/// let original = Mogulid::new(Ulid::new(), Ulid::new(), vec![Ulid::new(), Ulid::new()]);
/// let mut challenger_allowed = Mogulid::new(original.get_ulid(), Ulid::new(), original.get_history());
/// challenger_allowed.define_new_state();
///
/// let merged = mogulid_merge(&original, &challenger_allowed);
/// assert_eq!(merged.get_history().len(), original.get_history().len() + 1);
/// ```
pub fn mogulid_merge(_original:&Mogulid, _challenger:&Mogulid) -> Mogulid {
    Mogulid::new(Ulid::new(), Ulid::new(), vec![])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use ulid::Ulid;
        use crate::{MogulidError, Mogulid, mogulid_allow_update};

        let original = Mogulid { ulid: Ulid::new(), state: Ulid::new(), history: vec![Ulid::new(),Ulid::new()]};
        let challenger_not_same = Mogulid { ulid: Ulid::new(), state: Ulid::new(), history: vec![Ulid::new(),Ulid::new()]};
        assert_eq!(mogulid_allow_update(&original, &challenger_not_same), Err(MogulidError::NotVersionsOfTheSameObject));
    }
}
