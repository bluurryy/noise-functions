inspect_asm::cell_distance_3d_simd:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movaps xmm1, xmmword ptr [rdi]
	xorps xmm0, xmm0
	cmpleps xmm0, xmm1
	movaps xmm2, xmm0
	andnps xmm2, xmmword ptr [rip + .LCPI8_0]
	andps xmm0, xmmword ptr [rip + .LCPI8_1]
	orps xmm0, xmm2
	addps xmm0, xmm1
	cvttps2dq xmm2, xmm0
	movd ecx, xmm2
	lea r11d, [rcx - 1]
	inc ecx
	movss xmm0, dword ptr [rip + .LCPI8_2]
	cmp r11d, ecx
	jle .LBB_1
.LBB_19:
	sqrtss xmm0, xmm0
	addss xmm0, dword ptr [rip + .LCPI8_4]
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB_1:
	pshufd xmm3, xmm2, 85
	movd eax, xmm3
	lea r15d, [rax - 1]
	lea esi, [rax + 1]
	cmp r15d, esi
	jle .LBB_4
.LBB_2:
	xor eax, eax
	cmp r11d, ecx
	setl dl
	jge .LBB_19
	mov al, dl
	add r11d, eax
	cmp r11d, ecx
	jle .LBB_2
	jmp .LBB_19
.LBB_4:
	pshufd xmm2, xmm2, 238
	movd edi, xmm2
	lea edx, [rdi - 1]
	lea r8d, [rdi + 1]
	cmp edx, r8d
	jle .LBB_5
.LBB_14:
	xor eax, eax
	cmp r11d, ecx
	setl al
	mov edx, r15d
.LBB_15:
	xor edi, edi
	cmp edx, esi
	setl r8b
	jge .LBB_17
	mov dil, r8b
	add edx, edi
	cmp edx, esi
	jle .LBB_15
.LBB_17:
	cmp r11d, ecx
	jge .LBB_19
	add r11d, eax
	cmp r11d, ecx
	jle .LBB_14
	jmp .LBB_19
.LBB_5:
	imul r9d, r11d, 501125321
	imul eax, eax, 1136930381
	add eax, -1136930381
	mov dword ptr [rsp - 20], eax
	imul r10d, edi, 1720413743
	add r10d, -1720413743
	movss xmm0, dword ptr [rip + .LCPI8_2]
	lea rbx, [rip + .L__unnamed__0]
	movaps xmm2, xmmword ptr [rip + .LCPI8_3]
	mov qword ptr [rsp - 8], rcx
	mov dword ptr [rsp - 24], r15d
.LBB_6:
	xor eax, eax
	xorps xmm3, xmm3
	cvtsi2ss xmm3, r11d
	mov dword ptr [rsp - 12], r11d
	cmp r11d, ecx
	setl al
	mov dword ptr [rsp - 16], eax
	mov ecx, dword ptr [rsp - 20]
.LBB_7:
	xor r12d, r12d
	cmp r15d, esi
	setl r12b
	mov r14d, ecx
	xor r14d, r9d
	xorps xmm4, xmm4
	cvtsi2ss xmm4, r15d
	movlhps xmm4, xmm3
	shufps xmm4, xmm3, 226
	mov r11d, r10d
	mov edi, edx
.LBB_8:
	xor ebp, ebp
	cmp edi, r8d
	setl al
	mov r13d, r14d
	xor r13d, r11d
	xorps xmm5, xmm5
	cvtsi2ss xmm5, edi
	imul r13d, r13d, 813
	and r13d, 1020
	movaps xmm6, xmm4
	shufps xmm6, xmm5, 4
	subps xmm6, xmm1
	movaps xmm5, xmmword ptr [rbx + 4*r13]
	mulps xmm5, xmm2
	addps xmm5, xmm6
	mulps xmm5, xmm5
	movaps xmm6, xmm5
	shufps xmm6, xmm5, 85
	addss xmm6, xmm5
	movhlps xmm5, xmm5
	addss xmm5, xmm6
	minss xmm0, xmm5
	cmp edi, r8d
	jge .LBB_10
	mov bpl, al
	add edi, ebp
	add r11d, 1720413743
	cmp edi, r8d
	jle .LBB_8
.LBB_10:
	cmp r15d, esi
	jge .LBB_12
	add r15d, r12d
	add ecx, 1136930381
	cmp r15d, esi
	jle .LBB_7
.LBB_12:
	mov rcx, qword ptr [rsp - 8]
	mov r11d, dword ptr [rsp - 12]
	cmp r11d, ecx
	jge .LBB_19
	add r11d, dword ptr [rsp - 16]
	add r9d, 501125321
	cmp r11d, ecx
	mov r15d, dword ptr [rsp - 24]
	jle .LBB_6
	jmp .LBB_19