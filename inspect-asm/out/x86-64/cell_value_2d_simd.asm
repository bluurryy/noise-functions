inspect_asm::cell_value_2d_simd:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movsd xmm1, qword ptr [rdi]
	xorps xmm0, xmm0
	cmpleps xmm0, xmm1
	movaps xmm2, xmm0
	andnps xmm2, xmmword ptr [rip + .LCPI11_0]
	andps xmm0, xmmword ptr [rip + .LCPI11_1]
	orps xmm0, xmm2
	addps xmm0, xmm1
	cvttps2dq xmm2, xmm0
	movd eax, xmm2
	lea ecx, [rax - 1]
	inc eax
	xorps xmm0, xmm0
	cmp ecx, eax
	jg .LBB_11
	pshufd xmm2, xmm2, 85
	movd edi, xmm2
	lea edx, [rdi - 1]
	lea esi, [rdi + 1]
	mov dword ptr [rsp - 4], edx
	cmp edx, esi
	jle .LBB_2
.LBB_9:
	xor edx, edx
	cmp ecx, eax
	setl sil
	jge .LBB_11
	mov dl, sil
	add ecx, edx
	cmp ecx, eax
	jle .LBB_9
	jmp .LBB_11
.LBB_2:
	imul edi, edi, 1136930381
	add edi, -1136930381
	imul r8d, ecx, 501125321
	movss xmm2, dword ptr [rip + .LCPI11_2]
	xor r9d, r9d
	lea r10, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI11_3]
.LBB_3:
	xor r11d, r11d
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ecx
	cmp ecx, eax
	setl bl
	mov ebp, edi
	mov r14d, dword ptr [rsp - 4]
.LBB_4:
	movaps xmm4, xmm2
	xor r15d, r15d
	cmp r14d, esi
	setl r12b
	mov r13d, ebp
	xor r13d, r8d
	imul r13d, r13d, 668265261
	mov edx, r13d
	and edx, 510
	xorps xmm2, xmm2
	cvtsi2ss xmm2, r14d
	movsd xmm5, qword ptr [r10 + 4*rdx]
	movlhps xmm2, xmm3
	shufps xmm2, xmm3, 226
	subps xmm2, xmm1
	mulps xmm5, xmm0
	addps xmm5, xmm2
	mulps xmm5, xmm5
	movaps xmm2, xmm5
	shufps xmm2, xmm5, 85
	addss xmm2, xmm5
	ucomiss xmm4, xmm2
	cmova r9d, r13d
	minss xmm2, xmm4
	cmp r14d, esi
	jge .LBB_6
	mov r15b, r12b
	add r14d, r15d
	add ebp, 1136930381
	cmp r14d, esi
	jle .LBB_4
.LBB_6:
	cmp ecx, eax
	jge .LBB_8
	mov r11b, bl
	add ecx, r11d
	add r8d, 501125321
	cmp ecx, eax
	jle .LBB_3
.LBB_8:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r9d
	mulss xmm0, dword ptr [rip + .LCPI11_4]
.LBB_11:
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret