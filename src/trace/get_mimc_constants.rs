// Doing it this way would mean one constrain polynomial per constant which is too many ofr this demo so probably remove this file at the end

use rand::rngs::OsRng;
use crate::finite_field::field_params::{Fp, FpRepr};
use crate::ff::{Field, PrimeField};
use crate::MIMC_ROUNDS;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub fn generate_random_mimc_constants(size: u32) {
    for _ in 1..=size {
        let random_field_element = Fp::random(OsRng);
        println!("{:?}", random_field_element.to_repr().as_ref());
    }
}

// The function above was used to generate these 127 random field elements. Each one is used in one round of the MiMC hash function.
pub static MIMC_CONSTANTS: Lazy<[Fp; MIMC_ROUNDS as usize]> = Lazy::new(|| [
    Fp::from_repr(FpRepr([227, 190, 227, 225, 200, 156, 236, 3, 195, 252, 118, 37, 96, 137, 37, 7, 77, 135, 22, 156, 67, 213, 4, 3, 212, 248, 68, 119, 61, 185, 90, 0])).unwrap(),
    Fp::from_repr(FpRepr([142, 49, 6, 58, 54, 101, 5, 20, 92, 102, 245, 70, 210, 62, 69, 52, 76, 195, 165, 181, 173, 154, 10, 86, 64, 195, 251, 61, 167, 79, 127, 0])).unwrap(),
    Fp::from_repr(FpRepr([231, 68, 35, 134, 236, 214, 76, 111, 54, 215, 234, 237, 104, 203, 16, 199, 37, 57, 79, 51, 15, 217, 35, 130, 98, 237, 246, 191, 132, 242, 56, 0])).unwrap(),
    Fp::from_repr(FpRepr([158, 143, 239, 222, 197, 149, 145, 116, 20, 232, 102, 47, 123, 67, 168, 166, 195, 116, 39, 146, 169, 66, 231, 194, 197, 234, 152, 47, 10, 153, 19, 0])).unwrap(),
    Fp::from_repr(FpRepr([76, 224, 172, 159, 165, 7, 181, 242, 28, 29, 151, 233, 47, 94, 25, 243, 54, 3, 230, 29, 66, 94, 133, 141, 218, 0, 240, 83, 38, 203, 57, 0])).unwrap(),
    Fp::from_repr(FpRepr([122, 224, 248, 205, 193, 177, 19, 173, 151, 36, 251, 35, 160, 195, 116, 116, 174, 117, 5, 218, 122, 149, 63, 208, 166, 146, 244, 98, 222, 234, 91, 0])).unwrap(),
    Fp::from_repr(FpRepr([81, 246, 247, 64, 38, 64, 218, 29, 227, 195, 118, 243, 216, 193, 28, 43, 118, 128, 108, 161, 225, 245, 254, 80, 72, 221, 94, 24, 190, 200, 123, 0])).unwrap(),
    Fp::from_repr(FpRepr([192, 191, 141, 187, 188, 248, 104, 103, 221, 67, 120, 73, 237, 171, 22, 146, 6, 143, 35, 91, 85, 175, 81, 214, 173, 41, 116, 193, 43, 255, 91, 0])).unwrap(),
    Fp::from_repr(FpRepr([142, 206, 182, 185, 236, 127, 131, 146, 197, 229, 141, 51, 204, 239, 248, 244, 155, 221, 17, 235, 96, 158, 225, 204, 188, 230, 99, 68, 107, 97, 45, 0])).unwrap(),
    Fp::from_repr(FpRepr([134, 145, 146, 87, 185, 240, 41, 83, 111, 89, 151, 58, 79, 7, 60, 58, 34, 214, 247, 138, 252, 147, 146, 37, 188, 8, 242, 44, 151, 41, 39, 0])).unwrap(),
    Fp::from_repr(FpRepr([40, 202, 227, 93, 4, 75, 217, 27, 3, 25, 25, 72, 1, 69, 179, 2, 39, 224, 27, 103, 92, 119, 82, 214, 57, 0, 160, 182, 151, 124, 65, 0])).unwrap(),
    Fp::from_repr(FpRepr([4, 14, 112, 135, 30, 115, 166, 238, 102, 129, 219, 76, 184, 103, 7, 57, 43, 228, 224, 255, 131, 181, 124, 255, 104, 90, 223, 196, 187, 129, 110, 0])).unwrap(),
    Fp::from_repr(FpRepr([96, 129, 126, 123, 53, 111, 112, 96, 155, 20, 207, 2, 37, 238, 68, 163, 181, 45, 171, 160, 170, 28, 202, 59, 168, 93, 122, 197, 20, 237, 96, 0])).unwrap(),
    Fp::from_repr(FpRepr([108, 204, 46, 42, 31, 167, 118, 233, 29, 113, 39, 203, 54, 5, 195, 169, 9, 187, 92, 81, 234, 78, 154, 100, 93, 0, 169, 38, 147, 199, 70, 0])).unwrap(),
    Fp::from_repr(FpRepr([121, 206, 194, 212, 127, 253, 197, 125, 229, 132, 233, 6, 72, 187, 54, 87, 112, 235, 64, 27, 97, 253, 54, 19, 73, 135, 177, 199, 221, 78, 111, 0])).unwrap(),
    Fp::from_repr(FpRepr([17, 165, 233, 33, 56, 99, 5, 137, 73, 182, 242, 191, 24, 89, 77, 63, 195, 48, 95, 150, 10, 209, 118, 0, 207, 237, 131, 233, 241, 139, 49, 0])).unwrap(),
    Fp::from_repr(FpRepr([206, 4, 97, 153, 220, 161, 12, 139, 132, 157, 122, 222, 164, 83, 217, 12, 230, 66, 88, 252, 85, 36, 28, 97, 210, 229, 48, 247, 130, 198, 11, 0])).unwrap(),
    Fp::from_repr(FpRepr([119, 119, 58, 201, 107, 204, 34, 60, 79, 151, 240, 216, 3, 57, 233, 148, 200, 26, 124, 88, 145, 160, 221, 73, 19, 27, 115, 195, 52, 180, 40, 0])).unwrap(),
    Fp::from_repr(FpRepr([90, 71, 214, 77, 141, 225, 251, 227, 204, 237, 239, 65, 101, 202, 55, 153, 88, 45, 19, 246, 183, 52, 41, 200, 152, 142, 195, 159, 57, 190, 35, 0])).unwrap(),
    Fp::from_repr(FpRepr([214, 206, 97, 135, 139, 95, 65, 255, 149, 158, 177, 125, 89, 66, 180, 39, 94, 101, 96, 197, 199, 22, 123, 186, 224, 44, 150, 61, 11, 118, 85, 0])).unwrap(),
    Fp::from_repr(FpRepr([26, 133, 50, 169, 182, 103, 155, 238, 237, 166, 129, 137, 18, 213, 122, 170, 87, 220, 152, 27, 190, 53, 213, 180, 245, 100, 215, 92, 75, 138, 127, 0])).unwrap(),
    Fp::from_repr(FpRepr([23, 221, 189, 75, 1, 154, 103, 52, 23, 117, 148, 80, 147, 81, 236, 203, 187, 45, 81, 179, 177, 125, 15, 234, 11, 210, 211, 175, 35, 215, 94, 0])).unwrap(),
    Fp::from_repr(FpRepr([161, 25, 180, 164, 254, 43, 194, 25, 236, 33, 59, 139, 0, 29, 127, 171, 235, 124, 27, 194, 150, 78, 64, 39, 187, 237, 87, 26, 187, 86, 91, 0])).unwrap(),
    Fp::from_repr(FpRepr([95, 75, 152, 74, 247, 203, 251, 211, 40, 41, 227, 32, 255, 117, 119, 58, 191, 187, 142, 144, 206, 182, 69, 232, 181, 59, 2, 117, 128, 137, 44, 0])).unwrap(),
    Fp::from_repr(FpRepr([98, 174, 120, 88, 95, 3, 42, 90, 108, 81, 89, 251, 236, 100, 184, 26, 122, 125, 175, 116, 124, 171, 145, 36, 106, 249, 147, 81, 111, 157, 46, 0])).unwrap(),
    Fp::from_repr(FpRepr([180, 192, 88, 75, 0, 252, 9, 211, 38, 70, 201, 161, 79, 163, 199, 54, 23, 246, 134, 200, 56, 154, 26, 101, 89, 118, 223, 44, 0, 103, 84, 0])).unwrap(),
    Fp::from_repr(FpRepr([252, 162, 171, 82, 56, 247, 161, 230, 196, 62, 132, 181, 218, 172, 133, 99, 226, 143, 220, 74, 130, 174, 148, 115, 33, 163, 83, 101, 98, 149, 120, 0])).unwrap(),
    Fp::from_repr(FpRepr([246, 73, 168, 237, 96, 250, 176, 152, 214, 167, 147, 120, 148, 241, 170, 171, 15, 131, 149, 161, 23, 29, 66, 95, 159, 179, 175, 113, 16, 61, 99, 0])).unwrap(),
    Fp::from_repr(FpRepr([90, 140, 253, 248, 98, 180, 9, 208, 60, 74, 147, 194, 43, 240, 129, 14, 13, 217, 253, 165, 200, 124, 159, 116, 239, 194, 137, 55, 186, 197, 67, 0])).unwrap(),
    Fp::from_repr(FpRepr([119, 108, 28, 173, 155, 213, 5, 62, 205, 210, 183, 49, 244, 210, 250, 218, 170, 240, 138, 160, 31, 40, 248, 231, 35, 162, 206, 44, 41, 97, 113, 0])).unwrap(),
    Fp::from_repr(FpRepr([83, 104, 232, 47, 206, 79, 104, 237, 163, 148, 225, 241, 18, 30, 210, 192, 229, 4, 119, 90, 76, 4, 93, 36, 172, 83, 156, 141, 174, 187, 51, 0])).unwrap(),
    Fp::from_repr(FpRepr([221, 178, 222, 99, 116, 16, 123, 242, 126, 104, 196, 124, 189, 219, 109, 105, 37, 139, 180, 124, 208, 57, 140, 246, 179, 180, 139, 211, 146, 86, 110, 0])).unwrap(),
    Fp::from_repr(FpRepr([205, 230, 247, 46, 28, 58, 64, 98, 231, 72, 170, 80, 204, 226, 157, 100, 225, 6, 250, 2, 233, 254, 62, 72, 222, 140, 232, 139, 11, 168, 0, 0])).unwrap(),
    Fp::from_repr(FpRepr([194, 102, 106, 41, 21, 63, 246, 4, 122, 225, 209, 69, 188, 84, 124, 218, 209, 95, 46, 188, 102, 38, 49, 204, 228, 193, 197, 147, 40, 7, 93, 0])).unwrap(),
    Fp::from_repr(FpRepr([44, 85, 56, 134, 176, 104, 11, 77, 53, 31, 130, 30, 31, 184, 151, 121, 168, 134, 22, 17, 252, 169, 93, 241, 236, 91, 82, 85, 133, 243, 119, 0])).unwrap(),
    Fp::from_repr(FpRepr([18, 111, 87, 188, 80, 249, 43, 227, 49, 160, 172, 161, 28, 86, 81, 101, 175, 85, 105, 143, 250, 97, 253, 90, 104, 80, 240, 249, 60, 98, 6, 0])).unwrap(),
    Fp::from_repr(FpRepr([126, 210, 113, 85, 190, 85, 177, 65, 93, 235, 79, 239, 197, 58, 38, 220, 0, 16, 87, 33, 182, 125, 207, 217, 139, 161, 125, 17, 105, 217, 20, 0])).unwrap(),
    Fp::from_repr(FpRepr([228, 113, 165, 37, 139, 68, 79, 236, 151, 74, 78, 119, 100, 231, 14, 95, 76, 134, 12, 24, 187, 164, 149, 58, 107, 151, 250, 60, 234, 48, 63, 0])).unwrap(),
    Fp::from_repr(FpRepr([61, 159, 17, 10, 145, 20, 59, 39, 192, 61, 110, 251, 50, 188, 212, 26, 211, 93, 131, 207, 90, 228, 171, 234, 182, 93, 253, 241, 91, 108, 88, 0])).unwrap(),
    Fp::from_repr(FpRepr([49, 122, 179, 250, 118, 162, 57, 209, 52, 176, 205, 242, 56, 93, 80, 206, 196, 160, 61, 222, 83, 93, 198, 46, 113, 50, 72, 45, 152, 143, 12, 0])).unwrap(),
    Fp::from_repr(FpRepr([184, 49, 8, 205, 203, 100, 124, 225, 243, 183, 150, 47, 45, 14, 73, 177, 250, 97, 181, 77, 216, 22, 187, 72, 71, 29, 215, 31, 211, 65, 86, 0])).unwrap(),
    Fp::from_repr(FpRepr([99, 84, 198, 63, 231, 177, 59, 214, 232, 68, 122, 126, 193, 36, 244, 202, 25, 0, 147, 239, 53, 230, 166, 123, 144, 157, 236, 203, 112, 127, 68, 0])).unwrap(),
    Fp::from_repr(FpRepr([172, 155, 157, 140, 29, 81, 81, 74, 242, 252, 255, 78, 110, 233, 55, 224, 38, 21, 80, 133, 109, 93, 13, 6, 221, 83, 23, 42, 26, 53, 97, 0])).unwrap(),
    Fp::from_repr(FpRepr([30, 180, 233, 8, 171, 81, 13, 232, 103, 244, 244, 118, 132, 190, 74, 55, 212, 24, 246, 20, 52, 26, 136, 12, 31, 101, 131, 254, 165, 60, 88, 0])).unwrap(),
    Fp::from_repr(FpRepr([76, 205, 24, 221, 227, 23, 201, 125, 7, 44, 29, 57, 34, 203, 13, 200, 99, 125, 63, 55, 182, 121, 105, 42, 132, 246, 126, 81, 105, 58, 6, 0])).unwrap(),
    Fp::from_repr(FpRepr([125, 43, 131, 88, 171, 211, 242, 102, 125, 19, 24, 43, 175, 6, 188, 40, 227, 222, 7, 220, 88, 199, 251, 81, 167, 47, 87, 154, 97, 76, 86, 0])).unwrap(),
    Fp::from_repr(FpRepr([148, 184, 93, 115, 211, 64, 42, 150, 198, 134, 127, 102, 30, 67, 37, 74, 166, 150, 206, 58, 16, 2, 235, 144, 242, 62, 189, 196, 105, 246, 100, 0])).unwrap(),
    Fp::from_repr(FpRepr([15, 114, 39, 152, 39, 168, 161, 118, 69, 88, 158, 5, 100, 206, 249, 248, 153, 131, 85, 248, 202, 216, 146, 139, 196, 143, 159, 150, 141, 103, 102, 0])).unwrap(),
    Fp::from_repr(FpRepr([130, 168, 150, 70, 73, 122, 131, 8, 185, 219, 125, 198, 95, 135, 36, 50, 129, 2, 218, 171, 16, 79, 69, 254, 240, 120, 104, 66, 169, 79, 35, 0])).unwrap(),
    Fp::from_repr(FpRepr([209, 225, 223, 232, 13, 134, 131, 203, 223, 176, 38, 128, 162, 36, 18, 173, 119, 22, 184, 237, 159, 195, 112, 126, 123, 242, 145, 64, 1, 122, 57, 0])).unwrap(),
    Fp::from_repr(FpRepr([241, 96, 9, 109, 59, 17, 17, 243, 238, 187, 149, 83, 33, 12, 24, 193, 175, 4, 123, 160, 154, 61, 60, 49, 208, 205, 138, 104, 209, 194, 88, 0])).unwrap(),
    Fp::from_repr(FpRepr([129, 193, 52, 171, 137, 59, 155, 1, 154, 245, 168, 202, 228, 231, 38, 113, 145, 200, 57, 118, 205, 33, 194, 78, 120, 85, 196, 136, 163, 59, 54, 0])).unwrap(),
    Fp::from_repr(FpRepr([188, 51, 117, 192, 158, 74, 72, 30, 124, 168, 223, 38, 44, 139, 73, 5, 136, 115, 175, 204, 165, 246, 80, 209, 131, 12, 227, 139, 18, 221, 42, 0])).unwrap(),
    Fp::from_repr(FpRepr([54, 123, 0, 233, 48, 15, 8, 239, 208, 78, 250, 97, 185, 44, 38, 192, 213, 18, 243, 227, 166, 229, 33, 13, 76, 130, 123, 235, 222, 183, 32, 0])).unwrap(),
    Fp::from_repr(FpRepr([82, 189, 213, 135, 187, 65, 137, 177, 10, 11, 66, 191, 64, 194, 226, 25, 145, 144, 58, 87, 195, 212, 89, 186, 214, 87, 165, 142, 17, 177, 125, 0])).unwrap(),
    Fp::from_repr(FpRepr([12, 201, 107, 40, 13, 241, 238, 114, 110, 126, 74, 113, 97, 194, 86, 113, 39, 238, 198, 204, 141, 245, 157, 145, 86, 185, 62, 49, 139, 215, 80, 0])).unwrap(),
    Fp::from_repr(FpRepr([164, 175, 102, 160, 140, 191, 174, 73, 105, 50, 207, 103, 249, 162, 156, 23, 191, 131, 221, 37, 174, 154, 23, 137, 73, 214, 201, 61, 106, 65, 68, 0])).unwrap(),
    Fp::from_repr(FpRepr([104, 119, 148, 202, 220, 242, 68, 142, 147, 91, 95, 55, 236, 141, 62, 139, 37, 163, 236, 85, 230, 167, 109, 86, 4, 110, 168, 79, 62, 226, 1, 0])).unwrap(),
    Fp::from_repr(FpRepr([38, 27, 248, 5, 26, 20, 131, 161, 145, 183, 105, 234, 250, 5, 48, 99, 28, 142, 195, 61, 175, 155, 62, 224, 46, 10, 66, 238, 93, 212, 71, 0])).unwrap(),
    Fp::from_repr(FpRepr([111, 208, 41, 106, 249, 33, 75, 199, 248, 75, 207, 47, 28, 72, 161, 224, 9, 30, 160, 27, 4, 180, 67, 244, 229, 168, 2, 9, 70, 18, 111, 0])).unwrap(),
    Fp::from_repr(FpRepr([137, 128, 133, 238, 131, 250, 13, 23, 129, 20, 138, 161, 237, 130, 33, 41, 215, 96, 129, 69, 232, 173, 177, 172, 86, 34, 215, 219, 175, 19, 31, 0])).unwrap(),
    Fp::from_repr(FpRepr([44, 85, 94, 215, 179, 30, 91, 223, 221, 101, 173, 46, 163, 210, 87, 178, 30, 194, 173, 216, 58, 178, 130, 189, 215, 189, 15, 4, 228, 19, 16, 0])).unwrap(),
    Fp::from_repr(FpRepr([78, 237, 101, 223, 174, 198, 169, 149, 157, 61, 175, 109, 218, 44, 119, 97, 148, 15, 8, 34, 68, 199, 194, 93, 128, 197, 127, 133, 196, 114, 11, 0])).unwrap(),
    Fp::from_repr(FpRepr([128, 251, 73, 165, 189, 50, 159, 40, 126, 46, 104, 242, 99, 170, 73, 166, 107, 152, 227, 44, 41, 244, 91, 228, 249, 213, 190, 69, 90, 18, 108, 0])).unwrap(),
    Fp::from_repr(FpRepr([126, 62, 154, 106, 114, 19, 183, 107, 84, 162, 233, 35, 55, 219, 10, 139, 60, 249, 185, 202, 206, 88, 55, 81, 2, 58, 0, 14, 93, 206, 7, 0])).unwrap(),
    Fp::from_repr(FpRepr([128, 158, 48, 61, 163, 68, 67, 194, 151, 154, 88, 112, 120, 44, 161, 142, 76, 115, 176, 84, 146, 99, 254, 205, 140, 179, 3, 15, 42, 135, 113, 0])).unwrap(),
    Fp::from_repr(FpRepr([233, 59, 63, 124, 181, 24, 142, 228, 192, 217, 205, 210, 179, 225, 243, 166, 20, 155, 210, 132, 29, 73, 17, 227, 91, 146, 51, 69, 239, 254, 79, 0])).unwrap(),
    Fp::from_repr(FpRepr([96, 60, 209, 217, 24, 7, 154, 218, 15, 75, 30, 18, 38, 62, 121, 27, 37, 54, 1, 113, 204, 120, 56, 198, 129, 38, 26, 231, 249, 163, 126, 0])).unwrap(),
    Fp::from_repr(FpRepr([47, 243, 116, 49, 148, 26, 111, 188, 61, 239, 240, 53, 9, 63, 43, 92, 47, 93, 99, 211, 180, 167, 198, 186, 20, 188, 147, 215, 43, 208, 19, 0])).unwrap(),
    Fp::from_repr(FpRepr([235, 99, 52, 44, 37, 119, 181, 4, 91, 8, 45, 123, 118, 100, 255, 130, 86, 74, 208, 93, 236, 158, 227, 111, 221, 2, 110, 240, 2, 179, 32, 0])).unwrap(),
    Fp::from_repr(FpRepr([219, 254, 145, 65, 179, 23, 80, 225, 132, 252, 127, 231, 20, 145, 23, 61, 67, 163, 174, 212, 162, 35, 235, 253, 123, 163, 29, 3, 223, 140, 46, 0])).unwrap(),
    Fp::from_repr(FpRepr([194, 124, 123, 201, 156, 160, 74, 56, 223, 233, 31, 140, 200, 11, 137, 32, 191, 227, 1, 54, 151, 26, 206, 144, 7, 130, 20, 190, 32, 196, 127, 0])).unwrap(),
    Fp::from_repr(FpRepr([0, 243, 116, 19, 209, 67, 8, 198, 167, 253, 189, 179, 116, 42, 16, 218, 187, 118, 219, 160, 250, 11, 222, 158, 34, 99, 56, 93, 177, 229, 101, 0])).unwrap(),
    Fp::from_repr(FpRepr([227, 23, 250, 183, 37, 155, 83, 76, 121, 79, 79, 125, 108, 251, 106, 169, 51, 251, 177, 181, 131, 126, 26, 128, 45, 220, 3, 26, 64, 30, 10, 0])).unwrap(),
    Fp::from_repr(FpRepr([109, 122, 5, 65, 46, 121, 127, 188, 153, 132, 207, 86, 198, 236, 112, 115, 116, 125, 66, 220, 154, 116, 158, 154, 82, 222, 241, 154, 179, 205, 119, 0])).unwrap(),
    Fp::from_repr(FpRepr([160, 158, 64, 34, 90, 18, 154, 90, 246, 209, 107, 221, 17, 232, 227, 38, 104, 113, 150, 86, 53, 196, 126, 167, 158, 144, 233, 141, 188, 180, 65, 0])).unwrap(),
    Fp::from_repr(FpRepr([59, 43, 244, 152, 198, 22, 22, 135, 99, 174, 211, 254, 74, 121, 46, 109, 90, 80, 223, 9, 140, 76, 40, 52, 4, 188, 165, 136, 40, 134, 50, 0])).unwrap(),
    Fp::from_repr(FpRepr([183, 173, 161, 85, 3, 29, 94, 90, 172, 208, 161, 111, 16, 72, 19, 60, 138, 19, 144, 240, 106, 127, 214, 169, 87, 227, 106, 64, 106, 9, 39, 0])).unwrap(),
    Fp::from_repr(FpRepr([66, 160, 230, 75, 151, 110, 188, 101, 37, 66, 213, 120, 120, 217, 24, 135, 145, 138, 89, 127, 8, 98, 78, 89, 189, 173, 131, 245, 231, 1, 104, 0])).unwrap(),
    Fp::from_repr(FpRepr([118, 252, 141, 174, 249, 227, 111, 212, 225, 210, 64, 201, 157, 213, 180, 242, 77, 51, 191, 218, 127, 137, 27, 107, 71, 162, 152, 165, 26, 131, 102, 0])).unwrap(),
    Fp::from_repr(FpRepr([212, 95, 252, 54, 115, 13, 39, 24, 43, 38, 193, 2, 64, 24, 134, 80, 87, 73, 167, 168, 57, 37, 89, 143, 211, 148, 64, 201, 220, 115, 113, 0])).unwrap(),
    Fp::from_repr(FpRepr([91, 176, 208, 111, 179, 113, 254, 235, 160, 91, 48, 42, 12, 65, 67, 29, 23, 162, 199, 33, 52, 141, 91, 158, 148, 109, 102, 144, 162, 220, 25, 0])).unwrap(),
    Fp::from_repr(FpRepr([57, 151, 50, 108, 59, 218, 84, 20, 232, 179, 15, 180, 101, 155, 251, 39, 37, 145, 174, 3, 207, 199, 124, 183, 54, 179, 170, 21, 206, 206, 89, 0])).unwrap(),
    Fp::from_repr(FpRepr([202, 88, 104, 140, 8, 123, 158, 4, 121, 151, 177, 160, 158, 7, 54, 251, 214, 119, 37, 51, 172, 166, 212, 106, 188, 209, 240, 225, 206, 92, 127, 0])).unwrap(),
    Fp::from_repr(FpRepr([88, 207, 161, 153, 5, 91, 76, 51, 173, 150, 166, 40, 98, 110, 73, 22, 29, 255, 144, 108, 120, 229, 118, 44, 166, 147, 253, 46, 91, 9, 18, 0])).unwrap(),
    Fp::from_repr(FpRepr([4, 208, 143, 163, 179, 131, 68, 77, 71, 7, 55, 44, 131, 57, 2, 254, 163, 107, 89, 89, 23, 51, 84, 77, 137, 37, 130, 63, 22, 142, 73, 0])).unwrap(),
    Fp::from_repr(FpRepr([135, 67, 98, 54, 150, 184, 232, 99, 130, 199, 239, 104, 155, 82, 192, 93, 158, 18, 139, 90, 49, 12, 79, 244, 79, 134, 74, 45, 225, 116, 109, 0])).unwrap(),
    Fp::from_repr(FpRepr([143, 224, 47, 25, 146, 107, 91, 33, 72, 209, 57, 137, 92, 59, 105, 159, 244, 201, 145, 24, 130, 206, 176, 1, 105, 31, 186, 119, 119, 105, 105, 0])).unwrap(),
    Fp::from_repr(FpRepr([136, 229, 31, 246, 148, 185, 202, 71, 219, 73, 117, 218, 140, 175, 11, 35, 193, 19, 154, 105, 118, 133, 172, 5, 106, 87, 153, 249, 202, 73, 58, 0])).unwrap(),
    Fp::from_repr(FpRepr([78, 70, 230, 59, 246, 221, 58, 204, 24, 78, 81, 39, 130, 227, 44, 148, 215, 104, 141, 224, 147, 21, 88, 92, 234, 93, 26, 182, 165, 254, 20, 0])).unwrap(),
    Fp::from_repr(FpRepr([73, 8, 52, 81, 98, 102, 79, 242, 136, 232, 29, 92, 55, 170, 169, 156, 191, 79, 110, 120, 132, 242, 35, 57, 153, 9, 169, 76, 169, 144, 117, 0])).unwrap(),
    Fp::from_repr(FpRepr([227, 78, 215, 197, 35, 127, 216, 6, 53, 190, 120, 242, 172, 153, 32, 115, 208, 176, 158, 30, 104, 111, 47, 241, 127, 16, 34, 124, 143, 145, 122, 0])).unwrap(),
    Fp::from_repr(FpRepr([217, 159, 160, 79, 222, 16, 95, 82, 41, 182, 158, 146, 165, 17, 115, 242, 130, 239, 187, 242, 51, 240, 21, 185, 4, 104, 101, 141, 139, 182, 41, 0])).unwrap(),
    Fp::from_repr(FpRepr([231, 152, 20, 220, 185, 252, 105, 213, 80, 67, 71, 175, 43, 175, 233, 108, 216, 253, 44, 97, 44, 220, 246, 194, 32, 128, 197, 2, 58, 144, 74, 0])).unwrap(),
    Fp::from_repr(FpRepr([98, 143, 93, 166, 62, 70, 0, 45, 198, 103, 38, 140, 45, 188, 39, 116, 132, 206, 15, 231, 227, 51, 159, 210, 13, 197, 65, 219, 230, 94, 83, 0])).unwrap(),
    Fp::from_repr(FpRepr([115, 77, 134, 139, 165, 94, 152, 207, 19, 224, 10, 183, 25, 214, 235, 219, 14, 100, 73, 74, 111, 202, 57, 81, 26, 16, 232, 194, 138, 81, 109, 0])).unwrap(),
    Fp::from_repr(FpRepr([73, 92, 66, 206, 1, 95, 79, 120, 213, 53, 118, 215, 178, 63, 96, 90, 90, 143, 146, 240, 3, 125, 180, 57, 116, 192, 180, 171, 52, 96, 106, 0])).unwrap(),
    Fp::from_repr(FpRepr([165, 223, 55, 44, 39, 88, 196, 6, 187, 205, 11, 110, 120, 207, 84, 145, 71, 60, 164, 219, 60, 31, 166, 132, 121, 68, 78, 5, 91, 50, 79, 0])).unwrap(),
    Fp::from_repr(FpRepr([63, 45, 184, 48, 172, 121, 35, 160, 185, 134, 143, 104, 161, 209, 71, 210, 211, 97, 62, 34, 18, 63, 2, 119, 133, 255, 123, 102, 159, 211, 46, 0])).unwrap(),
    Fp::from_repr(FpRepr([157, 112, 98, 138, 49, 121, 122, 48, 121, 143, 149, 0, 17, 87, 43, 44, 152, 195, 67, 172, 103, 159, 198, 176, 111, 86, 165, 133, 189, 26, 72, 0])).unwrap(),
    Fp::from_repr(FpRepr([226, 172, 44, 136, 210, 141, 158, 57, 108, 167, 172, 215, 241, 7, 8, 33, 237, 215, 68, 213, 117, 150, 168, 46, 248, 54, 157, 64, 10, 65, 82, 0])).unwrap(),
    Fp::from_repr(FpRepr([25, 105, 20, 145, 27, 121, 39, 90, 43, 192, 97, 7, 120, 17, 33, 129, 73, 198, 5, 144, 149, 64, 84, 124, 230, 92, 127, 11, 58, 24, 24, 0])).unwrap(),
    Fp::from_repr(FpRepr([6, 199, 15, 179, 2, 119, 89, 222, 25, 28, 52, 236, 255, 31, 96, 26, 255, 9, 177, 6, 219, 42, 84, 93, 135, 31, 194, 65, 33, 237, 56, 0])).unwrap(),
    Fp::from_repr(FpRepr([15, 119, 230, 130, 229, 227, 112, 64, 149, 40, 177, 17, 8, 47, 117, 172, 156, 207, 109, 27, 121, 59, 176, 130, 23, 11, 151, 38, 165, 198, 114, 0])).unwrap(),
    Fp::from_repr(FpRepr([198, 104, 167, 145, 194, 82, 21, 54, 153, 221, 22, 126, 170, 50, 231, 173, 110, 141, 92, 140, 43, 92, 123, 56, 120, 81, 197, 157, 20, 112, 38, 0])).unwrap(),
    Fp::from_repr(FpRepr([149, 101, 232, 27, 154, 95, 217, 3, 182, 82, 43, 247, 229, 7, 23, 200, 99, 69, 51, 165, 226, 85, 210, 54, 57, 23, 141, 113, 141, 33, 70, 0])).unwrap(),
    Fp::from_repr(FpRepr([16, 155, 77, 105, 21, 18, 30, 139, 239, 10, 88, 30, 206, 205, 180, 29, 37, 253, 145, 97, 119, 231, 218, 87, 20, 80, 74, 150, 156, 213, 16, 0])).unwrap(),
    Fp::from_repr(FpRepr([245, 73, 162, 73, 189, 219, 243, 57, 180, 48, 51, 188, 58, 134, 176, 8, 197, 251, 17, 172, 88, 207, 200, 30, 92, 229, 112, 226, 113, 222, 44, 0])).unwrap(),
    Fp::from_repr(FpRepr([214, 196, 4, 229, 237, 149, 197, 23, 38, 68, 14, 121, 3, 195, 236, 95, 29, 166, 106, 210, 178, 23, 44, 240, 44, 181, 253, 234, 176, 201, 10, 0])).unwrap(),
    Fp::from_repr(FpRepr([127, 242, 199, 45, 181, 39, 196, 250, 130, 78, 88, 208, 98, 118, 143, 99, 109, 6, 136, 224, 184, 74, 150, 47, 174, 149, 165, 224, 218, 245, 107, 0])).unwrap(),
    Fp::from_repr(FpRepr([13, 152, 2, 30, 83, 138, 9, 48, 220, 221, 50, 58, 131, 161, 138, 254, 204, 43, 138, 156, 240, 106, 225, 49, 12, 33, 216, 9, 167, 133, 6, 0])).unwrap(),
    Fp::from_repr(FpRepr([246, 47, 232, 13, 242, 64, 111, 99, 193, 81, 13, 211, 151, 34, 49, 214, 148, 105, 29, 250, 171, 22, 125, 84, 95, 76, 185, 33, 216, 174, 66, 0])).unwrap(),
    Fp::from_repr(FpRepr([242, 22, 223, 153, 254, 201, 39, 24, 126, 245, 54, 13, 113, 191, 52, 4, 63, 241, 55, 175, 136, 79, 184, 107, 108, 7, 207, 30, 158, 81, 125, 0])).unwrap(),
    Fp::from_repr(FpRepr([86, 61, 44, 98, 131, 118, 186, 65, 66, 145, 30, 27, 170, 12, 42, 82, 57, 141, 189, 87, 221, 124, 231, 2, 79, 95, 200, 157, 208, 77, 78, 0])).unwrap(),
    Fp::from_repr(FpRepr([238, 238, 63, 21, 124, 242, 61, 234, 124, 81, 140, 144, 36, 23, 112, 120, 144, 157, 17, 69, 173, 17, 106, 18, 221, 239, 242, 29, 14, 121, 2, 0])).unwrap(),
    Fp::from_repr(FpRepr([227, 80, 206, 240, 198, 101, 213, 127, 146, 173, 156, 96, 73, 103, 221, 208, 197, 79, 213, 27, 146, 156, 241, 77, 237, 46, 39, 45, 2, 148, 5, 0])).unwrap(),
    Fp::from_repr(FpRepr([163, 227, 207, 217, 50, 78, 51, 132, 14, 229, 232, 127, 63, 249, 36, 214, 84, 238, 154, 82, 213, 79, 144, 53, 63, 242, 248, 237, 114, 202, 75, 0])).unwrap(),
    Fp::from_repr(FpRepr([183, 53, 252, 105, 21, 19, 158, 82, 58, 21, 194, 97, 45, 27, 87, 220, 142, 135, 0, 118, 24, 22, 61, 255, 194, 46, 19, 36, 196, 151, 50, 0])).unwrap(),
    Fp::from_repr(FpRepr([170, 12, 207, 130, 64, 170, 40, 181, 152, 46, 166, 163, 200, 43, 50, 111, 129, 168, 138, 122, 234, 185, 211, 7, 186, 171, 183, 19, 208, 66, 78, 0])).unwrap(),
    Fp::from_repr(FpRepr([53, 98, 236, 146, 240, 117, 176, 147, 165, 120, 195, 248, 227, 254, 249, 34, 79, 86, 216, 24, 111, 142, 139, 82, 254, 157, 185, 20, 68, 135, 8, 0])).unwrap(),
    Fp::from_repr(FpRepr([36, 165, 105, 221, 58, 215, 145, 58, 136, 91, 88, 37, 167, 26, 195, 221, 10, 200, 57, 111, 48, 98, 43, 33, 1, 109, 59, 110, 216, 185, 18, 0])).unwrap(),
    Fp::from_repr(FpRepr([110, 227, 126, 250, 135, 251, 81, 88, 104, 146, 57, 161, 104, 150, 122, 221, 29, 253, 50, 127, 76, 129, 60, 242, 139, 5, 143, 22, 67, 76, 38, 0])).unwrap(),
    Fp::from_repr(FpRepr([7, 207, 89, 145, 19, 193, 92, 238, 56, 252, 205, 80, 31, 209, 191, 203, 148, 166, 50, 176, 215, 201, 90, 112, 150, 242, 85, 114, 87, 98, 38, 0])).unwrap(),
    Fp::from_repr(FpRepr([251, 149, 216, 138, 134, 247, 129, 163, 77, 241, 158, 8, 57, 195, 101, 83, 137, 80, 184, 185, 132, 196, 33, 65, 150, 247, 203, 69, 177, 107, 3, 0])).unwrap(),
    Fp::from_repr(FpRepr([92, 159, 126, 106, 57, 78, 27, 207, 200, 174, 205, 50, 127, 201, 225, 153, 21, 170, 250, 195, 235, 206, 16, 62, 107, 160, 61, 33, 186, 164, 26, 0])).unwrap(),
    Fp::from_repr(FpRepr([198, 147, 233, 55, 125, 151, 30, 192, 148, 242, 24, 46, 0, 242, 51, 93, 136, 77, 7, 128, 10, 88, 187, 189, 52, 77, 124, 19, 125, 180, 44, 0])).unwrap(),
    Fp::from_repr(FpRepr([203, 28, 119, 56, 8, 90, 200, 84, 135, 255, 185, 242, 217, 114, 20, 237, 62, 204, 170, 128, 35, 83, 241, 128, 207, 6, 246, 224, 20, 103, 83, 0])).unwrap(),
]);


