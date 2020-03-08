import {CollectionViewer, SelectionChange, DataSource} from '@angular/cdk/collections';
import {FlatTreeControl} from '@angular/cdk/tree';
import {TreeControl} from '@angular/cdk/tree';
import { Component, OnInit, Injectable } from '@angular/core';
import {BehaviorSubject, merge, Observable} from 'rxjs';
import {map} from 'rxjs/operators';
import { MatDialogRef } from '@angular/material/dialog';
import { MusicfilesService, MusicFileEntry, FileType } from '../../musicfiles.service';

export class FileNode {
  constructor(public item: MusicFileEntry, public level = 1, public expandable = false,
              public isLoading = false, ) {}
}

Injectable({
  providedIn: 'root'
})
export class FileDatabase {
  
  rootLevelNodes: MusicFileEntry[]

  constructor() {
    let item = new MusicFileEntry();
    item.file_name = 'ROOT';
    item.file_type = FileType.Directory;
    item.parent_path = "";
    this.rootLevelNodes = [item]
  }

  /** Initial data from database */
  initialData(): FileNode[] {
    return this.rootLevelNodes.map(n => new FileNode(n, 0, true));
  }

  getChildren(node: MusicFileEntry, fileService: MusicfilesService): Observable<MusicFileEntry[]> | undefined {
    if (node.file_name === 'ROOT') {
      return fileService.getRoot()
    } else {
      if (node.parent_path) {
        return fileService.getSub(node.parent_path + node.separator.toString() + node.file_name);
      }
      return fileService.getSub(node.file_name);
    }
  }

  isExpandable(node: MusicFileEntry): boolean {
    return node.file_type == FileType.Directory;
  }
}

export class FileDataSource {

  dataChange = new BehaviorSubject<FileNode[]>([]);

  get data(): FileNode[] { return this.dataChange.value; }
  set data(value: FileNode[]) {
    this.treeControl.dataNodes = value;
    this.dataChange.next(value);
  }

  constructor(private treeControl: FlatTreeControl<FileNode>,
              private database: FileDatabase,
              private fileService: MusicfilesService) {}

  connect(collectionViewer: CollectionViewer): Observable<FileNode[]> {
    this.treeControl.expansionModel.onChange.subscribe(change => {
      if ((change as SelectionChange<FileNode>).added ||
        (change as SelectionChange<FileNode>).removed) {
        this.handleTreeControl(change as SelectionChange<FileNode>);
      }
    });

    return merge(collectionViewer.viewChange, this.dataChange).pipe(map(() => this.data));
  }

  /** Handle expand/collapse behaviors */
  handleTreeControl(change: SelectionChange<FileNode>) {
    console.debug('test');
    if (change.added) {
      change.added.forEach(node => this.toggleNode(node, true));
    }
    if (change.removed) {
      change.removed.slice().reverse().forEach(node => this.toggleNode(node, false));
    }
  }

  /**
   * Toggle the node, remove from display list
   */
  toggleNode(node: FileNode, expand: boolean) {
    console.debug('trace');
    const children = this.database.getChildren(node.item, this.fileService);
    const index = this.data.indexOf(node);
    if (!children || index < 0) { // If no children, or cannot find the node, no op
      return;
    }

    node.isLoading = true;

    if (expand) {
      children.subscribe(music_files => {
        if (music_files) {
          const nodes = music_files.map(music_file => {
            return new FileNode(music_file, node.level + 1, this.database.isExpandable(music_file))});

          this.data.splice(index + 1, 0, ...nodes);
          // notify the change
          this.dataChange.next(this.data);
          node.isLoading = false;
        }
      });
    } else {
      let count = 0;
      for (let i = index + 1; i < this.data.length
        && this.data[i].level > node.level; i++, count++) {}
      this.data.splice(index + 1, count);
      // notify the change
      this.dataChange.next(this.data);
      node.isLoading = false;
    }
  }
}

@Component({
  selector: 'app-file-selector',
  templateUrl: './file-selector.component.html',
  styleUrls: ['./file-selector.component.scss'],
  providers: [FileDatabase]
})
export class FileSelectorComponent implements OnInit {

  treeControl: FlatTreeControl<FileNode>;

  dataSource: FileDataSource;

  baseDir: String;

  constructor(
    private database: FileDatabase,
    public dialogRef: MatDialogRef<FileSelectorComponent>, 
    private fileService: MusicfilesService
  ) { 
    this.treeControl = new FlatTreeControl<FileNode>(this.getLevel, this.isExpandable);
    this.dataSource = new FileDataSource(this.treeControl, this.database, this.fileService);
    this.dataSource.data = this.database.initialData();
  }

  ngOnInit() {
  }

  getLevel = (node: FileNode) => node.level;

  isExpandable = (node: FileNode) => node.expandable;

  hasChild = (_: number, _nodeData: FileNode) => _nodeData.expandable

  selected(node: FileNode) {
    let result = node.item.parent_path ? node.item.parent_path + '/' + node.item.file_name : node.item.file_name;
    this.dialogRef.close(result);
  }
}
