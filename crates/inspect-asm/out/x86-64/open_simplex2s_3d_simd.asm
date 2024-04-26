inspect_asm::open_simplex2s_3d_simd:
	sub rsp, 24
	movaps xmm0, xmmword ptr [rdi]
	movlps qword ptr [rsp + 8], xmm0
	movhlps xmm0, xmm0
	movss dword ptr [rsp + 16], xmm0
	lea rdi, [rsp + 8]
	call noise_functions::scalar::open_simplex_2s::gen3
	add rsp, 24
	ret