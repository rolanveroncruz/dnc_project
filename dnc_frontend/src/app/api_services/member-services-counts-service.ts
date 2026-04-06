import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';
import {MasterListMemberLookupResponse} from './master-list-members-service';

export interface MemberServicesCountsRow {
    dental_service_id: number,
    dental_service_name: string,
    dental_service_type_id: number,
    record_tooth: boolean,
    counts_allowed: number,
    counts_used: number,
    has_pending: boolean,
    conflict_date: Date | null,

}
export interface MemberServicesCountsSummary extends MemberServicesCountsRow {
    verification_counts_allowed: number,
}
@Injectable({
  providedIn: 'root',
})
export class MemberServicesCountsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }
    private readonly baseUrl = `${environment.apiBaseUrl}/api`;

    getMemberServicesCountsSummary(memberId: number):Observable<MemberServicesCountsSummary[]>{
        return this.http.get<MemberServicesCountsRow[]>(`${this.baseUrl}/master_list_members/${memberId}/service_counts`,
            { headers:this.authHeaders()}
        )
            .pipe(
                map ( rows=>
                rows.map(row=>({
                    ...row,
                    verification_counts_allowed: row.record_tooth? 3:1,
                })
                ))
            )
    }
}
