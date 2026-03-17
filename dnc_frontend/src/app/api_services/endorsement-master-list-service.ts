import {inject, Injectable} from '@angular/core';

import {
    ExistingMasterListMeta, MasterListPreview
} from '../features/setup/setup-endorsements/setup-endorsements-component/endorsement-master-list-upload-component/data-types';
import {map, Observable} from 'rxjs';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';

interface MasterListMetaDataResponse {
    id: number;
    file_name: string;
    uploaded_by: string | null;
    upload_date: string | null;
    total_rows: number | null;
}
interface EndorsementMasterListMemberResponse {
    file_name: string;
    master_list_member_id: number;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    is_active: boolean;
}

interface MasterListMemberResponse {
    id: number;
    master_list_id: number;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    email_address: string;
    mobile_number: string | null;
    birth_date: string | null;
    is_active: boolean;
}


@Injectable({
  providedIn: 'root',
})
export class EndorsementMasterListService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = `${environment.apiBaseUrl}`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getEndorsementMasterListMeta(id: number): Observable<ExistingMasterListMeta> {
        return this.http.get<MasterListMetaDataResponse>(`${this.baseUrl}/api/endorsements/${id}/master_list_metadata`, {headers: this.authHeaders()})
            .pipe(map(meta => ({
                master_list_id: meta.id,
                last_file_uploaded: meta.file_name,
                uploaded_by: meta.uploaded_by ?? '',
                uploaded_at: meta.upload_date ?? '',
                total_rows: meta.total_rows?? 0,
            })))
    }

    uploadEndorsementMasterList(endorsement_id: number, file: File): Observable<MasterListPreview> {
        const formData = new FormData();
        formData.append('file', file, file.name);
        return this.http.post<MasterListPreview>(`${this.baseUrl}/api/endorsements/${endorsement_id}/master_list`,
            formData,
            {headers: this.authHeaders()});
    }

    deleteEndorsementMasterList(eid: number): Observable<any> {
        return this.http.delete<any>(`${this.baseUrl}/api/endorsements/${eid}/master_list`, {headers: this.authHeaders()});

    }

    getMasterListForEndorsement( endorsementId: number ): Observable<EndorsementMasterListMemberResponse[]> {
        return this.http.get<EndorsementMasterListMemberResponse[]>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/master_lists`,
            { headers: this.authHeaders() }
        );
    }

    setMasterListMemberActive( masterListMemberId: number, isActive: boolean ): Observable<MasterListMemberResponse> {
        return this.http.patch<MasterListMemberResponse>( `${this.baseUrl}/api/master_list_members/${masterListMemberId}/active`,
             { is_active: isActive }, { headers: this.authHeaders() } );
    }
}
