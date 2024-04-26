inspect_asm::cell_distance_sq_2d_simd:
	push rbp
	push r15
	push r14
	push rbx
	movsd xmm1, qword ptr [rdi]
	xorps xmm0, xmm0
	cmpleps xmm0, xmm1
	movaps xmm2, xmm0
	andnps xmm2, xmmword ptr [rip + .LCPI3_0]
	andps xmm0, xmmword ptr [rip + .LCPI3_1]
	orps xmm0, xmm2
	addps xmm0, xmm1
	cvttps2dq xmm2, xmm0
	movd eax, xmm2
	lea ecx, [rax - 1]
	inc eax
	movss xmm0, dword ptr [rip + .LCPI3_2]
	cmp ecx, eax
	jg .LBB_11
	pshufd xmm2, xmm2, 85
	movd edi, xmm2
	lea edx, [rdi - 1]
	lea esi, [rdi + 1]
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
	movss xmm0, dword ptr [rip + .LCPI3_2]
	lea r9, [rip + .L__unnamed__0]
	movaps xmm2, xmmword ptr [rip + .LCPI3_3]
.LBB_3:
	xor r10d, r10d
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ecx
	cmp ecx, eax
	setl r10b
	mov r11d, edi
	mov ebx, edx
.LBB_4:
	xor ebp, ebp
	cmp ebx, esi
	setl r14b
	mov r15d, r11d
	xor r15d, r8d
	imul r15d, r15d, 301
	and r15d, 510
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ebx
	movsd xmm5, qword ptr [r9 + 4*r15]
	movlhps xmm4, xmm3
	shufps xmm4, xmm3, 226
	subps xmm4, xmm1
	mulps xmm5, xmm2
	addps xmm5, xmm4
	mulps xmm5, xmm5
	movaps xmm4, xmm5
	shufps xmm4, xmm5, 85
	addss xmm4, xmm5
	minss xmm0, xmm4
	cmp ebx, esi
	jge .LBB_6
	mov bpl, r14b
	add ebx, ebp
	add r11d, 1136930381
	cmp ebx, esi
	jle .LBB_4
.LBB_6:
	cmp ecx, eax
	jge .LBB_8
	add ecx, r10d
	add r8d, 501125321
	cmp ecx, eax
	jle .LBB_3
.LBB_8:
	addss xmm0, dword ptr [rip + .LCPI3_4]
.LBB_11:
	pop rbx
	pop r14
	pop r15
	pop rbp
	ret