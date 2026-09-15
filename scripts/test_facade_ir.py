from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen
from sdk_ir import FacadeIr, JsonRequest, JsonResponse, NoRequest, ParametersRequest
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class ResolvedFacadeIrTests(unittest.TestCase):
    def test_build_ir_lowers_to_closed_operation_projections(self):
        rust = sdk_codegen.RustIndex(TYPES.encode(), CLIENT.encode())
        ir = sdk_codegen.build_ir(sdk_codegen.OpenApiIndex(openapi_document()), rust, manifest())

        self.assertIsInstance(ir, FacadeIr)
        operation = ir.resources[0].operations[0]
        self.assertIsInstance(operation.request_projection, JsonRequest)
        self.assertIsInstance(operation.response_projection, JsonResponse)
        self.assertEqual(operation.request, "Adoption")
        self.assertEqual(operation.request_raw, "AnimalRequest")
        self.assertEqual(operation.response, "Receipt")
        self.assertEqual(operation.raw_signature.success_type, "AnimalResponse")
        self.assertEqual(
            [(parameter.name, parameter.type) for parameter in operation.raw_signature.parameters],
            [("request", "AnimalRequest")],
        )

    def test_request_algebra_distinguishes_no_request_from_parameters(self):
        no_request = sdk_codegen._resolved_ir(
            sdk_codegen.SdkIr(
                "Client",
                (),
                (sdk_codegen.ResourceSpec(
                    ("health",),
                    "health",
                    "Health",
                    (sdk_codegen.OperationSpec("get", "get", "get", None, "Receipt", (), None),),
                ),),
            ),
            sdk_codegen.RustIndex(
                b"pub struct AnimalResponse { pub id: String }",
                b"impl HttpClient { pub async fn get(&self) -> Result<AnimalResponse, Error> { todo!() } }",
            ),
        )
        self.assertIsInstance(no_request.resources[0].operations[0].request_projection, NoRequest)

        parameters = sdk_codegen._resolved_ir(
            sdk_codegen.SdkIr(
                "Client",
                (),
                (sdk_codegen.ResourceSpec(
                    ("health",),
                    "health",
                    "Health",
                    (sdk_codegen.OperationSpec("get", "get", "get", None, "Receipt", (), None),),
                ),),
            ),
            sdk_codegen.RustIndex(
                b"pub struct AnimalResponse { pub id: String }",
                b"impl HttpClient { pub async fn get(&self, id: impl AsRef<str>) -> Result<AnimalResponse, Error> { todo!() } }",
            ),
        )
        self.assertIsInstance(parameters.resources[0].operations[0].request_projection, ParametersRequest)


if __name__ == "__main__":
    unittest.main()
