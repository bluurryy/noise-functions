inspect_asm::cell_distance_sq_3d:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movss xmm1, dword ptr [rdi]
	xorps xmm2, xmm2
	cmpnless xmm2, xmm1
	movss xmm0, dword ptr [rip + .LCPI2_0]
	movaps xmm3, xmm2
	andps xmm3, xmm0
	movss xmm4, dword ptr [rip + .LCPI2_1]
	andnps xmm2, xmm4
	orps xmm2, xmm3
	addss xmm2, xmm1
	cvttss2si r11d, xmm2
	movss xmm5, dword ptr [rip + .LCPI2_2]
	ucomiss xmm2, xmm5
	mov eax, 2147483647
	cmova r11d, eax
	xor edx, edx
	ucomiss xmm2, xmm2
	cmovp r11d, edx
	movss xmm2, dword ptr [rdi + 4]
	xorps xmm6, xmm6
	cmpnless xmm6, xmm2
	movaps xmm3, xmm6
	andps xmm3, xmm0
	andnps xmm6, xmm4
	orps xmm6, xmm3
	addss xmm6, xmm2
	cvttss2si ecx, xmm6
	ucomiss xmm6, xmm5
	cmova ecx, eax
	movss xmm3, dword ptr [rdi + 8]
	ucomiss xmm6, xmm6
	xorps xmm6, xmm6
	cmovp ecx, edx
	cmpnless xmm6, xmm3
	andps xmm0, xmm6
	andnps xmm6, xmm4
	orps xmm6, xmm0
	addss xmm6, xmm3
	cvttss2si edi, xmm6
	ucomiss xmm6, xmm5
	cmova edi, eax
	ucomiss xmm6, xmm6
	cmovp edi, edx
	lea edx, [r11 - 1]
	inc r11d
	movss xmm0, dword ptr [rip + .LCPI2_3]
	cmp edx, r11d
	jle .LBB_1
.LBB_20:
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB_1:
	lea r15d, [rcx - 1]
	inc ecx
	cmp r15d, ecx
	jle .LBB_4
.LBB_2:
	xor eax, eax
	cmp edx, r11d
	setl cl
	jge .LBB_20
	mov al, cl
	add edx, eax
	cmp edx, r11d
	jle .LBB_2
	jmp .LBB_20
.LBB_4:
	lea esi, [rdi - 1]
	inc edi
	cmp esi, edi
	jle .LBB_5
.LBB_15:
	xor eax, eax
	cmp edx, r11d
	setl al
	mov esi, r15d
.LBB_16:
	xor edi, edi
	cmp esi, ecx
	setl r8b
	jge .LBB_18
	mov dil, r8b
	add esi, edi
	cmp esi, ecx
	jle .LBB_16
.LBB_18:
	cmp edx, r11d
	jge .LBB_20
	add edx, eax
	cmp edx, r11d
	jle .LBB_15
	jmp .LBB_20
.LBB_5:
	imul r9d, edx, 501125321
	imul eax, r15d, 1136930381
	mov dword ptr [rsp - 24], eax
	imul r10d, esi, 1720413743
	movss xmm0, dword ptr [rip + .LCPI2_3]
	lea rbx, [rip + .L__unnamed__0]
	movss xmm4, dword ptr [rip + .LCPI2_4]
	movaps xmm5, xmmword ptr [rip + .LCPI2_5]
	mov qword ptr [rsp - 8], r11
	mov dword ptr [rsp - 20], r15d
.LBB_6:
	xor eax, eax
	cmp edx, r11d
	mov dword ptr [rsp - 12], edx
	xorps xmm6, xmm6
	cvtsi2ss xmm6, edx
	setl al
	mov dword ptr [rsp - 16], eax
	subss xmm6, xmm1
	mov edx, dword ptr [rsp - 24]
.LBB_7:
	xor r12d, r12d
	cmp r15d, ecx
	setl r12b
	mov r14d, edx
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r15d
	xor r14d, r9d
	subss xmm7, xmm2
	mov r11d, r10d
	mov r8d, esi
.LBB_8:
	xor ebp, ebp
	cmp r8d, edi
	setl al
	mov r13d, r14d
	xor r13d, r11d
	imul r13d, r13d, 813
	and r13d, 1020
	movss xmm8, dword ptr [rbx + 4*r13 + 4]
	mulss xmm8, xmm4
	xorps xmm9, xmm9
	cvtsi2ss xmm9, r8d
	addss xmm8, xmm7
	subss xmm9, xmm3
	mulss xmm8, xmm8
	movss xmm10, dword ptr [rbx + 4*r13 + 8]
	movss xmm11, dword ptr [rbx + 4*r13]
	unpcklps xmm11, xmm10
	mulps xmm11, xmm5
	movlhps xmm9, xmm6
	shufps xmm9, xmm6, 226
	addps xmm9, xmm11
	mulps xmm9, xmm9
	addss xmm8, xmm9
	shufps xmm9, xmm9, 85
	addss xmm9, xmm8
	minss xmm0, xmm9
	cmp r8d, edi
	jge .LBB_10
	mov bpl, al
	add r8d, ebp
	add r11d, 1720413743
	cmp r8d, edi
	jle .LBB_8
.LBB_10:
	cmp r15d, ecx
	jge .LBB_12
	add r15d, r12d
	add edx, 1136930381
	cmp r15d, ecx
	jle .LBB_7
.LBB_12:
	mov r11, qword ptr [rsp - 8]
	mov edx, dword ptr [rsp - 12]
	cmp edx, r11d
	mov r15d, dword ptr [rsp - 20]
	jge .LBB_14
	add edx, dword ptr [rsp - 16]
	add r9d, 501125321
	cmp edx, r11d
	jle .LBB_6
.LBB_14:
	addss xmm0, dword ptr [rip + .LCPI2_6]
	jmp .LBB_20