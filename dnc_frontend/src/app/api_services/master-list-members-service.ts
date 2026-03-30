import { inject, Injectable } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

export interface MasterListMemberLookupResponse {
    master_list_member_id: number;
    endorsement_id: number | null;
    endorsement_agreement_corp_number: string | null;
    master_list_member_account_no: string;
    master_list_member_last_name: string;
    master_list_member_first_name: string;
    master_list_member_middle_name: string;
    master_list_member_name: string;
    master_list_member_email_address: string | null;
    master_list_member_mobile_number: string | null;
    master_list_member_birth_date: string | null;
    master_list_member_is_active: boolean;
}
interface MasterListMemberRow{
    id: number,
    endorsement_company_name: string,
    agreement_corp_number: string | null,
    endorsement_id: number,
    master_list_id: number|null,
    account_number: string,
    last_name: string,
    first_name: string,
    middle_name: string,
    email_address: string | null,
    mobile_number: string |null,
    birth_date: string| null,
    is_active: boolean,
}


/** Matches Rust CreateMasterListMemberRequest */
export interface CreateMasterListMemberRequest {
    endorsement_id: number;
    master_list_id?: number | null;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    email_address?: string | null;
    mobile_number?: string | null;
    birth_date?: string | null; // YYYY-MM-DD
    is_active: boolean;
    last_edited_by?: string | null;
}

/** Matches Rust PatchMasterListMemberRequest */
export interface PatchMasterListMemberRequest {
    endorsement_id?: number;
    master_list_id?: number | null;
    account_number?: string;
    last_name?: string;
    first_name?: string;
    middle_name?: string;
    email_address?: string | null;
    mobile_number?: string | null;
    birth_date?: string | null; // YYYY-MM-DD
    is_active?: boolean;
    last_edited_by?: string | null;
}


export interface MasterListMemberMutationResponse {
    id: number;
    master_list_id: number | null;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    email_address: string | null;
    mobile_number: string | null;
    birth_date: string | null;
    is_active: boolean;
}

@Injectable({
    providedIn: 'root',
})
export class MasterListMemberService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    // Adjust this if your backend uses a different prefix
    private readonly baseUrl = `${environment.apiBaseUrl}/api/master_list_members`;

    getMasterListMembersForEndorsement(endorsement_id: number): Observable<MasterListMemberLookupResponse[]>{
        return this.http.get<MasterListMemberRow[] >(`${environment.apiBaseUrl}/api/endorsements/${endorsement_id}/master_list_members`,
            { headers:this.authHeaders()})
            .pipe(map(members => members.map(member => ({
                master_list_member_id: member.id,
                endorsement_id: endorsement_id,
                endorsement_agreement_corp_number: member.agreement_corp_number ?? null,
                master_list_member_account_no: member.account_number,
                master_list_member_last_name: member.last_name,
                master_list_member_first_name: member.first_name,
                master_list_member_middle_name: member.middle_name,
                master_list_member_name: member.last_name + ', ' + member.first_name + ' ' + member.middle_name,
                master_list_member_email_address: member.email_address,
                master_list_member_mobile_number: member.mobile_number,
                master_list_member_birth_date: member.birth_date ?? null,
                master_list_member_is_active: member.is_active,
            }))))

            ;
    }

    createMasterListMember(
        payload: CreateMasterListMemberRequest
    ): Observable<MasterListMemberMutationResponse> {
        return this.http.post<MasterListMemberMutationResponse>(
            this.baseUrl,
            payload,
            { headers: this.authHeaders() });
    }

    patchMasterListMember(
        id: number,
        payload: PatchMasterListMemberRequest
    ): Observable<MasterListMemberMutationResponse> {
        return this.http.patch<MasterListMemberMutationResponse>(
            `${this.baseUrl}/${id}`,
            payload,
            { headers: this.authHeaders() }
        );
    }
}
